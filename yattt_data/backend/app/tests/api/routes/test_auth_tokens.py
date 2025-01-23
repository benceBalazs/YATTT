import uuid

from fastapi.testclient import TestClient
from sqlmodel import Session

from app.core.config import settings

from ...utils.auth_token import create_random_auth_token

def test_create_auth_token(
    client: TestClient, superuser_token_headers: dict[str, str]
) -> None:
    data = {"tag_id": "Foo", "device_id": "Fighters"}
    response = client.post(
        f"{settings.API_V1_STR}/auth-tokens/",
        headers=superuser_token_headers,
        json=data,
    )
    assert response.status_code == 200
    content = response.json()
    assert content["tag_id"] == data["tag_id"]
    assert content["device_id"] == data["device_id"]
    assert "id" in content
    assert "scanned_in" in content


def test_read_auth_token(
    client: TestClient, superuser_token_headers: dict[str, str], db: Session
) -> None:
    auth_token = create_random_auth_token(db)
    response = client.get(
        f"{settings.API_V1_STR}/auth-tokens/{auth_token.id}",
        headers=superuser_token_headers,
    )
    assert response.status_code == 200
    content = response.json()
    assert content["tag_id"] == auth_token.tag_id
    assert content["device_id"] == auth_token.device_id
    assert content["id"] == str(auth_token.id)
    assert content["scanned_in"] == str(auth_token.scanned_in)


def test_read_auth_token_not_found(
    client: TestClient, superuser_token_headers: dict[str, str]
) -> None:
    response = client.get(
        f"{settings.API_V1_STR}/auth-tokens/{uuid.uuid4()}",
        headers=superuser_token_headers,
    )
    assert response.status_code == 404
    content = response.json()
    assert content["detail"] == "Auth Token not found"


def test_read_auth_token_not_enough_permissions(
    client: TestClient, normal_user_token_headers: dict[str, str], db: Session
) -> None:
    auth_token = create_random_auth_token(db)
    response = client.get(
        f"{settings.API_V1_STR}/auth-tokens/{auth_token.id}",
        headers=normal_user_token_headers,
    )
    assert response.status_code == 400
    content = response.json()
    assert content["detail"] == "Not enough permissions"


def test_read_auth_tokens(
    client: TestClient, superuser_token_headers: dict[str, str], db: Session
) -> None:
    create_random_auth_token(db)
    create_random_auth_token(db)
    response = client.get(
        f"{settings.API_V1_STR}/auth-tokens/",
        headers=superuser_token_headers,
    )
    assert response.status_code == 200
    content = response.json()
    assert len(content["data"]) >= 2


def test_update_auth_token(
    client: TestClient, superuser_token_headers: dict[str, str], db: Session
) -> None:
    auth_token = create_random_auth_token(db)
    data = {"tag_id": "TagId", "device_id": "DeviceId"}
    response = client.put(
        f"{settings.API_V1_STR}/auth-tokens/{auth_token.id}",
        headers=superuser_token_headers,
        json=data,
    )
    assert response.status_code == 200
    content = response.json()
    assert content["tag_id"] == data["tag_id"]
    assert content["device_id"] == data["device_id"]
    assert content["id"] == str(auth_token.id)


def test_update_auth_token_not_found(
    client: TestClient, superuser_token_headers: dict[str, str]
) -> None:
    data = {"tag_id": "TagId", "device_id": "DeviceId"}
    response = client.put(
        f"{settings.API_V1_STR}/auth-tokens/{uuid.uuid4()}",
        headers=superuser_token_headers,
        json=data,
    )
    assert response.status_code == 404
    content = response.json()
    assert content["detail"] == "Auth Token not found"


def test_update_auth_tokens_not_enough_permissions(
    client: TestClient, normal_user_token_headers: dict[str, str], db: Session
) -> None:
    auth_token = create_random_auth_token(db)
    data = {"tag_id": "TagId", "device_id": "DeviceId"}
    response = client.put(
        f"{settings.API_V1_STR}/auth-tokens/{auth_token.id}",
        headers=normal_user_token_headers,
        json=data,
    )
    assert response.status_code == 400
    content = response.json()
    assert content["detail"] == "Not enough permissions"


def test_delete_auth_token(
    client: TestClient, superuser_token_headers: dict[str, str], db: Session
) -> None:
    auth_token = create_random_auth_token(db)
    response = client.delete(
        f"{settings.API_V1_STR}/auth-tokens/{auth_token.id}",
        headers=superuser_token_headers,
    )
    assert response.status_code == 200
    content = response.json()
    assert content["message"] == "Auth Token deleted successfully"


def test_delete_auth_token_not_found(
    client: TestClient, superuser_token_headers: dict[str, str]
) -> None:
    response = client.delete(
        f"{settings.API_V1_STR}/auth-tokens/{uuid.uuid4()}",
        headers=superuser_token_headers,
    )
    assert response.status_code == 404
    content = response.json()
    assert content["detail"] == "Auth Token not found"


def test_delete_auth_token_not_enough_permissions(
    client: TestClient, normal_user_token_headers: dict[str, str], db: Session
) -> None:
    auth_token = create_random_auth_token(db)
    response = client.delete(
        f"{settings.API_V1_STR}/auth-tokens/{auth_token.id}",
        headers=normal_user_token_headers,
    )
    assert response.status_code == 400
    content = response.json()
    assert content["detail"] == "Not enough permissions"
