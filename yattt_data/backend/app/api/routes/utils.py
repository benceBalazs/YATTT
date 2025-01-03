from fastapi import APIRouter, Depends

router = APIRouter()


@router.get("/health-check/")
async def health_check() -> bool:
    return True
