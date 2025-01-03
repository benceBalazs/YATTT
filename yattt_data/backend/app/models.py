from datetime import datetime
from pydantic import EmailStr
from sqlmodel import Field, Relationship, SQLModel


# Shared properties
class UserBase(SQLModel):
    email: EmailStr = Field(unique=True, index=True, max_length=255)
    is_active: bool = True
    is_superuser: bool = False
    full_name: str | None = Field(default=None, max_length=255)


# Properties to receive via API on creation
class UserCreate(UserBase):
    password: str = Field(min_length=8, max_length=40)


class UserRegister(SQLModel):
    email: EmailStr = Field(max_length=255)
    password: str = Field(min_length=8, max_length=40)
    full_name: str | None = Field(default=None, max_length=255)


# Properties to receive via API on update, all are optional
class UserUpdate(UserBase):
    email: EmailStr | None = Field(default=None, max_length=255)  # type: ignore
    password: str | None = Field(default=None, min_length=8, max_length=40)


class UserUpdateMe(SQLModel):
    full_name: str | None = Field(default=None, max_length=255)
    email: EmailStr | None = Field(default=None, max_length=255)


class UpdatePassword(SQLModel):
    current_password: str = Field(min_length=8, max_length=40)
    new_password: str = Field(min_length=8, max_length=40)


# Database model, database table inferred from class name
class User(UserBase, table=True):
    id: int = Field(default=None, primary_key=True)
    hashed_password: str


# Properties to return via API, id is always required
class UserPublic(UserBase):
    id: int


class UsersPublic(SQLModel):
    data: list[UserPublic]
    count: int


# Shared properties
class AuthTokenBase(SQLModel):
    tag_id: str
    device_id: str
    scanned_in: datetime | None = None
    scanned_out: datetime | None = None
    description: str | None = None


# Properties to receive on item creation
class AuthTokenCreate(AuthTokenBase):
    tag_id: str
    device_id: str
    scanned_in: datetime | None = None
    scanned_out: datetime | None = None

class AuthTokenScanned(AuthTokenBase):
    tag_id: str
    device_id: str

# Properties to receive on item update
class AuthTokenUpdate(AuthTokenBase):
    tag_id: str | None = None  # type: ignore
    device_id: str | None = None  # type: ignore


# Database model, database table inferred from class name
class AuthToken(AuthTokenBase, table=True):
    id: int | None = Field(default=None, primary_key=True)
    tag_id: str
    device_id: str
    scanned_in: datetime | None = Field(default_factory=lambda: datetime.now())
    scanned_out: datetime | None = None


# Properties to return via API, id is always required
class AuthTokenPublic(AuthTokenBase):
    id: int

class AuthTokensPublic(SQLModel):
    data: list[AuthTokenPublic]
    count: int

class AuthTokenScanned(AuthTokenBase):
    tag_id: str
    device_id: str

# Generic message
class Message(SQLModel):
    message: str


# JSON payload containing access token
class Token(SQLModel):
    access_token: str
    token_type: str = "bearer"


# Contents of JWT token
class TokenPayload(SQLModel):
    sub: str | None = None


class NewPassword(SQLModel):
    token: str
    new_password: str = Field(min_length=8, max_length=40)
