import requests
from datetime import datetime, timezone
from typing import Any

from fastapi import APIRouter, HTTPException
from sqlmodel import func, select

from app.api.deps import CurrentUser, SessionDep

from ...core.config import settings
from ...models import AuthTokensPublic, AuthToken, AuthTokenPublic, AuthTokenCreate, AuthTokenUpdate, AuthTokenScanned, Message

router = APIRouter()


@router.get("/", response_model=AuthTokensPublic)
def read_auth_tokens(
    session: SessionDep, current_user: CurrentUser, skip: int = 0, limit: int = 100
) -> Any:
    """
    Retrieve scanned auth tokens.
    """

    if current_user.is_superuser:
        count_statement = select(func.count()).select_from(AuthToken)
        count = session.exec(count_statement).one()
        statement = select(AuthToken).offset(skip).limit(limit)
        items = session.exec(statement).all()
    else:
        count_statement = (
            select(func.count())
            .select_from(AuthToken)
        )
        count = session.exec(count_statement).one()
        statement = (
            select(AuthToken)
            .offset(skip)
            .limit(limit)
        )
        items = session.exec(statement).all()

    return AuthTokensPublic(data=items, count=count)


@router.get("/{id}", response_model=AuthTokenPublic)
def read_auth_token(session: SessionDep, current_user: CurrentUser, id: int) -> Any:
    """
    Get auth token by ID.
    """
    auth_token = session.get(AuthToken, id)
    if not auth_token:
        raise HTTPException(status_code=404, detail="Auth Token not found")
    if not current_user.is_superuser:
        raise HTTPException(status_code=400, detail="Not enough permissions")
    return auth_token


@router.post("/", response_model=AuthTokenPublic)
def create_auth_token(
    *, session: SessionDep, current_user: CurrentUser, auth_token_in: AuthTokenCreate
) -> Any:
    """
    Create new auth_token.
    """
    auth_token = AuthToken.model_validate(auth_token_in)
    session.add(auth_token)
    session.commit()
    session.refresh(auth_token)
    return auth_token


@router.put("/{id}", response_model=AuthTokenPublic)
def update_auth_token(
    *,
    session: SessionDep,
    current_user: CurrentUser,
    id: int,
    item_in: AuthTokenUpdate,
) -> Any:
    """
    Update an auth token.
    """
    auth_token = session.get(AuthToken, id)
    if not auth_token:
        raise HTTPException(status_code=404, detail="Auth Token not found")
    if not current_user.is_superuser:
        raise HTTPException(status_code=400, detail="Not enough permissions")
    update_dict = item_in.model_dump(exclude_unset=True)
    auth_token.sqlmodel_update(update_dict)
    session.add(auth_token)
    session.commit()
    session.refresh(auth_token)
    return auth_token


@router.delete("/{id}")
def delete_auth_token(
    session: SessionDep, current_user: CurrentUser, id: int
) -> Message:
    """
    Delete an item.
    """
    auth_token = session.get(AuthToken, id)
    if not auth_token:
        raise HTTPException(status_code=404, detail="Auth Token not found")
    if not current_user.is_superuser:
        raise HTTPException(status_code=400, detail="Not enough permissions")
    session.delete(auth_token)
    session.commit()
    return Message(message="Auth Token deleted successfully")

@router.post("/scan")
def scan(session: SessionDep, tag_info: AuthTokenScanned) -> Message:
    """
    Handle both scanning in and scanning out of an auth token.
    - If the token is already scanned in and not scanned out, scan it out.
    - If the token is not scanned in, scan it in.
    """
    # Print the received values for debugging
    print(f"Received tagId: {tag_info.tag_id}, deviceId: {tag_info.device_id}")

    # Fetch the auth token from the database
    db_auth_token = session.execute(
        select(AuthToken)
        .where(AuthToken.tag_id == tag_info.tag_id)
        .where(AuthToken.device_id == tag_info.device_id)
        .order_by(AuthToken.id.desc())
    ).scalars().first()

    if db_auth_token and db_auth_token.scanned_out is None:
        # Token is already scanned in, perform scan out
        db_auth_token.scanned_out = datetime.now(timezone.utc)
        try:
            session.add(db_auth_token)
            session.commit()

            data = {
                "tag_id": tag_info.tag_id,
                "device_id": tag_info.device_id,
                "check_in_time": db_auth_token.scanned_in.isoformat(),
                "check_out_time": db_auth_token.scanned_out.isoformat(),
                "duration": (db_auth_token.scanned_out - db_auth_token.scanned_in).total_seconds()
            }

            response = requests.post(
                settings.YATTT_BACKEND_URL + "/attendances",
                json=data,
                headers={"Authorization": "Bearer "+settings.YATTT_BACKEND_KEY}
            )
            response.raise_for_status()


        except Exception as e:
            session.rollback()
            print(f"Error updating database: {e}")
            raise HTTPException(status_code=500, detail="Failed to update the database.")

        return Message(message="SCANNED_OUT")

    else:
        # Token is not scanned in, perform scan in
        new_auth_token = AuthToken(tag_id=tag_info.tag_id, device_id=tag_info.device_id)
        try:
            session.add(new_auth_token)
            session.commit()
            session.refresh(new_auth_token)
        except Exception as e:
            session.rollback()
            print(f"Error inserting into database: {e}")
            raise HTTPException(status_code=500, detail="Failed to insert into the database.")

        return Message(message="SCANNED_IN")
