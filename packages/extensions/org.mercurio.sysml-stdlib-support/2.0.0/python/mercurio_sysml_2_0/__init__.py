"""Generated Mercurio wrappers for sysml-2.0-pilot-0.57.0."""

from .base import ElementView, StdlibRef
from .metamodel import (
    METAMODEL_CLASSES,
    METAMODEL_CLASS_BY_KIND,
    METAMODEL_CLASS_BY_METATYPE,
    class_for,
    wrap,
)
from .concepts import (
    AttributeUsage,
    ConstraintUsage,
    MetadataUsage,
    Package,
    PartDefinition,
    PartUsage,
    RequirementUsage,
    SysML,
    VerificationCaseUsage,
)
from .generation_info import KIR_SCHEMA_VERSION, PROFILE_ID, STDLIB_VERSION

__all__ = [
    "AttributeUsage",
    "ConstraintUsage",
    "ElementView",
    "KIR_SCHEMA_VERSION",
    "METAMODEL_CLASSES",
    "METAMODEL_CLASS_BY_KIND",
    "METAMODEL_CLASS_BY_METATYPE",
    "MetadataUsage",
    "StdlibRef",
    "Package",
    "PartDefinition",
    "PartUsage",
    "PROFILE_ID",
    "RequirementUsage",
    "STDLIB_VERSION",
    "SysML",
    "VerificationCaseUsage",
    "class_for",
    "register",
    "wrap",
]

for _cls in METAMODEL_CLASSES:
    globals().setdefault(_cls.__name__, _cls)
    if _cls.__name__ not in __all__:
        __all__.append(_cls.__name__)
del _cls


def register(registry):
    registry.register_profile("sysml-2.0-pilot-0.57.0", "mercurio_sysml_2_0")
    registry.register("package", Package)
    registry.register("part_definition", PartDefinition)
    registry.register("part_usage", PartUsage)
    registry.register("attribute_usage", AttributeUsage)
    registry.register("requirement_usage", RequirementUsage)
    registry.register("verification_case_usage", VerificationCaseUsage)
    registry.register("constraint_usage", ConstraintUsage)
    registry.register("metadata_usage", MetadataUsage)
    register_metamodel = getattr(registry, "register_metamodel_classes", None)
    if callable(register_metamodel):
        register_metamodel(METAMODEL_CLASSES)
    register_metatype = getattr(registry, "register_metatype", None)
    if callable(register_metatype):
        for metatype_id, cls in METAMODEL_CLASS_BY_METATYPE.items():
            register_metatype(metatype_id, cls)
    register_kind = getattr(registry, "register_kind", None)
    if callable(register_kind):
        for kind, cls in METAMODEL_CLASS_BY_KIND.items():
            register_kind(kind, cls)
