from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any, ClassVar


class ElementView:
    concept: ClassVar[str | None] = None
    metatype_id: ClassVar[str | None] = None
    metatype_ids: ClassVar[tuple[str, ...]] = ()
    kind_name: ClassVar[str | None] = None
    kind_names: ClassVar[tuple[str, ...]] = ()

    def __init__(self, element: Any):
        self._element = element

    @classmethod
    def wrap(cls, element: Any):
        if isinstance(element, cls):
            return element
        return cls(element)

    @classmethod
    def matches(cls, element: Any) -> bool:
        metatype_id = getattr(element, "metatype_id", None)
        if callable(metatype_id):
            metatype_id = metatype_id()
        kind = getattr(element, "kind", None)
        if callable(kind):
            kind = kind()
        metatype_ids = cls.metatype_ids or ((cls.metatype_id,) if cls.metatype_id else ())
        kind_names = cls.kind_names or ((cls.kind_name,) if cls.kind_name else ())
        return (
            bool(metatype_id and metatype_id in metatype_ids)
            or bool(kind and kind in kind_names)
            or (not metatype_ids and not kind_names and cls.metatype_id is None)
        )

    @property
    def id(self) -> str:
        return self._element.id

    @property
    def kind(self) -> str:
        return self._element.kind

    def get(self, name: str) -> Any:
        get = getattr(self._element, "get", None)
        if callable(get):
            return get(name)
        get_json = getattr(self._element, "get_json", None)
        if callable(get_json):
            value = get_json(name)
            return json.loads(value) if value is not None else None
        get_str = getattr(self._element, "get_str", None)
        if callable(get_str):
            return get_str(name)
        return None

    def effective(self, name: str) -> Any:
        effective = getattr(self._element, "effective", None)
        if callable(effective):
            return effective(name)
        effective_json = getattr(self._element, "effective_json", None)
        if callable(effective_json):
            value = effective_json(name)
            return json.loads(value) if value is not None else None
        effective_str = getattr(self._element, "effective_str", None)
        if callable(effective_str):
            return effective_str(name)
        return self.get(name)

    def references(self, name: str) -> list[Any]:
        return self._element.references(name)

    def metadata(self) -> Any:
        metadata = getattr(self._element, "metadata", None)
        return metadata() if callable(metadata) else metadata

    def metadata_by_type(self, type_name: str) -> list[Any]:
        metadata_by_type = getattr(self._element, "metadata_by_type", None)
        if callable(metadata_by_type):
            return metadata_by_type(type_name)
        metadata = self.metadata() or []
        return [
            item
            for item in metadata
            if getattr(item, "type_name", None) == type_name
            or getattr(item, "type", None) == type_name
        ]

    def effective_str(self, name: str) -> str | None:
        value = self.effective(name)
        return value if isinstance(value, str) else None


@dataclass(frozen=True)
class StdlibRef:
    id: str

    def bind(self, model: Any) -> Any:
        return model.element(self.id)
