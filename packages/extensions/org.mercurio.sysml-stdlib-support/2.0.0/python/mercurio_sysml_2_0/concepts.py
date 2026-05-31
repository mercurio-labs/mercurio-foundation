from __future__ import annotations

from . import metamodel as _metamodel
from .base import ElementView
from .stdlib.si import SINamespace
from .stdlib.isq import ISQNamespace


class Package(_metamodel.Package):
    concept = "package"
    metatype_id = "KerML::Kernel::Package"

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    def owned_members(self) -> list[ElementView]:
        return self.references("members") or self.references("features")


class PartDefinition(_metamodel.PartDefinition):
    concept = "part_definition"
    metatype_id = "SysML::Systems::PartDefinition"

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    def features(self) -> list[ElementView]:
        return self.references("features")


class PartUsage(_metamodel.PartUsage):
    concept = "part_usage"
    metatype_id = "SysML::Systems::PartUsage"

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")


class AttributeUsage(_metamodel.AttributeUsage):
    concept = "attribute_usage"
    metatype_id = "SysML::Systems::AttributeUsage"

    @property
    def name(self) -> str | None:
        return self.effective_str("name")


class RequirementUsage(_metamodel.RequirementUsage):
    concept = "requirement_usage"
    metatype_id = "SysML::Systems::RequirementUsage"

    @property
    def text(self) -> str | None:
        return self.effective_str("text") or self.effective_str("documentation")


class VerificationCaseUsage(_metamodel.VerificationCaseUsage):
    concept = "verification_case_usage"
    metatype_id = "SysML::Systems::VerificationCaseUsage"

    @property
    def name(self) -> str | None:
        return self.effective_str("name")


class ConstraintUsage(_metamodel.ConstraintUsage):
    concept = "constraint_usage"
    metatype_id = "SysML::Systems::ConstraintUsage"

    @property
    def expression(self):
        return self.effective("expression")


class MetadataUsage(ElementView):
    concept = "metadata_usage"
    metatype_id = None

    @property
    def metadata_type(self) -> str | None:
        return self.effective_str("metadata_type") or self.effective_str("type")


class StdlibNamespace:
    def __init__(self, model):
        self.SI = SINamespace(model)
        self.ISQ = ISQNamespace(model)


class SysML:
    def __init__(self, model):
        self.model = model
        self.stdlib = StdlibNamespace(model)

    @classmethod
    def bind(cls, model):
        return cls(model)

    def elements_with_metadata(self, metadata_type: str) -> list[ElementView]:
        query = getattr(self.model, "elements_with_metadata", None)
        if callable(query):
            return query(metadata_type)
        return []
