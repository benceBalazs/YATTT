from sqlmodel import Session

from app import crud
from app.tests.utils.utils import random_lower_string

from app.models import AuthToken, AuthTokenCreate


def create_random_auth_token(db: Session) -> AuthToken:
    tag_id = random_lower_string()
    device_id = random_lower_string()
    auth_token_in = AuthTokenCreate(device_id=device_id, tag_id=tag_id)
    return crud.create_auth_token(session=db, auth_token_in=auth_token_in)
