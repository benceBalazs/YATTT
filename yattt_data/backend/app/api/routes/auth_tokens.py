import uuid
from typing import Any

from fastapi import APIRouter, HTTPException
from sqlmodel import func, select

from app.api.deps import CurrentUser, SessionDep

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
def read_auth_token(session: SessionDep, current_user: CurrentUser, id: uuid.UUID) -> Any:
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
    id: uuid.UUID,
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
    session: SessionDep, current_user: CurrentUser, id: uuid.UUID
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

@router.post("/scanin")
def scan_in(session: SessionDep, tag_info: AuthTokenScanned) -> Message:
    """
    Scan in an auth token(create it in the db), check if is already scanned in or not allowed.
    """
    # Print the received values
    print(f"Received tagId: {tag_info.tag_id}, deviceId: {tag_info.device_id}")

    db_auth_token = session.execute(select(AuthToken).where(AuthToken.tag_id == tag_info.tag_id)).first()
    # Placeholder logic for determining response status
    if db_auth_token:
        return Message(status="ALREADY_SCANNED")
    elif tag_info.device_id == "another_specific_value":
        return Message(status="NOT_ALLOWED")
    else:
        new_auth_token = AuthToken(tag_id=tag_info.tag_id, device_id=tag_info.device_id)
        session.add(new_auth_token)
        session.commit()
        session.refresh(new_auth_token)
        return Message(status="SCANNED_IN")
