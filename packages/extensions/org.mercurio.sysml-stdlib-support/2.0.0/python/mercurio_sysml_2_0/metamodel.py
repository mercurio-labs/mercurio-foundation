from __future__ import annotations

from typing import Any

from .base import ElementView


class Element(ElementView):
    concept = None
    metatype_id = "KerML::Root::Element"
    metatype_ids = ("KerML::Root::Element",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Namespace(Element):
    concept = None
    metatype_id = "KerML::Root::Namespace"
    metatype_ids = ("KerML::Root::Namespace",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Type(Namespace):
    concept = None
    metatype_id = "KerML::Core::Type"
    metatype_ids = ("KerML::Core::Type",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Feature(Type):
    concept = None
    metatype_id = "KerML::Core::Feature"
    metatype_ids = ("KerML::Core::Feature",)
    kind_name = "Feature"
    kind_names = ("Feature",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Usage(Feature):
    concept = None
    metatype_id = "SysML::Systems::Usage"
    metatype_ids = ("SysML::Systems::Usage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class OccurrenceUsage(Usage):
    concept = None
    metatype_id = "SysML::Systems::OccurrenceUsage"
    metatype_ids = ("SysML::Systems::OccurrenceUsage",)
    kind_name = "OccurrenceUsage"
    kind_names = ("OccurrenceUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Step(Feature):
    concept = None
    metatype_id = "KerML::Kernel::Step"
    metatype_ids = ("KerML::Kernel::Step",)
    kind_name = "Step"
    kind_names = ("Step",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ActionUsage(OccurrenceUsage, Step):
    concept = None
    metatype_id = "SysML::Systems::ActionUsage"
    metatype_ids = ("SysML::Systems::ActionUsage",)
    kind_name = "ActionUsage"
    kind_names = ("ActionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AcceptActionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::AcceptActionUsage"
    metatype_ids = ("SysML::Systems::AcceptActionUsage",)
    kind_name = "AcceptActionUsage"
    kind_names = ("AcceptActionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Classifier(Type):
    concept = None
    metatype_id = "KerML::Core::Classifier"
    metatype_ids = ("KerML::Core::Classifier",)
    kind_name = "Classifier"
    kind_names = ("Classifier",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Class(Classifier):
    concept = None
    metatype_id = "KerML::Kernel::Class"
    metatype_ids = ("KerML::Kernel::Class",)
    kind_name = "Class"
    kind_names = ("Class",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Behavior(Class):
    concept = None
    metatype_id = "KerML::Kernel::Behavior"
    metatype_ids = ("KerML::Kernel::Behavior",)
    kind_name = "Behavior"
    kind_names = ("Behavior",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Definition(Classifier):
    concept = None
    metatype_id = "SysML::Systems::Definition"
    metatype_ids = ("SysML::Systems::Definition",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class OccurrenceDefinition(Class, Definition):
    concept = None
    metatype_id = "SysML::Systems::OccurrenceDefinition"
    metatype_ids = ("SysML::Systems::OccurrenceDefinition",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ActionDefinition(Behavior, OccurrenceDefinition):
    concept = None
    metatype_id = "SysML::Systems::ActionDefinition"
    metatype_ids = ("SysML::Systems::ActionDefinition",)
    kind_name = "ActionDefinition"
    kind_names = ("ActionDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Relationship(Element):
    concept = None
    metatype_id = "KerML::Root::Relationship"
    metatype_ids = ("KerML::Root::Relationship",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Membership(Relationship):
    concept = None
    metatype_id = "KerML::Root::Membership"
    metatype_ids = ("KerML::Root::Membership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class OwningMembership(Membership):
    concept = None
    metatype_id = "KerML::Root::OwningMembership"
    metatype_ids = ("KerML::Root::OwningMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureMembership(OwningMembership):
    concept = None
    metatype_id = "KerML::Core::FeatureMembership"
    metatype_ids = ("KerML::Core::FeatureMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ParameterMembership(FeatureMembership):
    concept = None
    metatype_id = "KerML::Kernel::ParameterMembership"
    metatype_ids = ("KerML::Kernel::ParameterMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ActorMembership(ParameterMembership):
    concept = None
    metatype_id = "SysML::Systems::ActorMembership"
    metatype_ids = ("SysML::Systems::ActorMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Association(Classifier, Relationship):
    concept = None
    metatype_id = "KerML::Kernel::Association"
    metatype_ids = ("KerML::Kernel::Association",)
    kind_name = "Association"
    kind_names = ("Association",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Structure(Class):
    concept = None
    metatype_id = "KerML::Kernel::Structure"
    metatype_ids = ("KerML::Kernel::Structure",)
    kind_name = "Structure"
    kind_names = ("Structure",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AssociationStructure(Association, Structure):
    concept = None
    metatype_id = "KerML::Kernel::AssociationStructure"
    metatype_ids = ("KerML::Kernel::AssociationStructure",)
    kind_name = "AssociationStructure"
    kind_names = ("AssociationStructure",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ItemDefinition(OccurrenceDefinition, Structure):
    concept = None
    metatype_id = "SysML::Systems::ItemDefinition"
    metatype_ids = ("SysML::Systems::ItemDefinition",)
    kind_name = "ItemDefinition"
    kind_names = ("ItemDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PartDefinition(ItemDefinition):
    concept = None
    metatype_id = "SysML::Systems::PartDefinition"
    metatype_ids = ("SysML::Systems::PartDefinition",)
    kind_name = "PartDefinition"
    kind_names = ("PartDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConnectionDefinition(AssociationStructure, PartDefinition):
    concept = None
    metatype_id = "SysML::Systems::ConnectionDefinition"
    metatype_ids = ("SysML::Systems::ConnectionDefinition",)
    kind_name = "ConnectionDefinition"
    kind_names = ("ConnectionDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AllocationDefinition(ConnectionDefinition):
    concept = None
    metatype_id = "SysML::Systems::AllocationDefinition"
    metatype_ids = ("SysML::Systems::AllocationDefinition",)
    kind_name = "AllocationDefinition"
    kind_names = ("AllocationDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Connector(Feature, Relationship):
    concept = None
    metatype_id = "KerML::Kernel::Connector"
    metatype_ids = ("KerML::Kernel::Connector",)
    kind_name = "Connector"
    kind_names = ("Connector",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConnectorAsUsage(Connector, Usage):
    concept = None
    metatype_id = "SysML::Systems::ConnectorAsUsage"
    metatype_ids = ("SysML::Systems::ConnectorAsUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ItemUsage(OccurrenceUsage):
    concept = None
    metatype_id = "SysML::Systems::ItemUsage"
    metatype_ids = ("SysML::Systems::ItemUsage",)
    kind_name = "ItemUsage"
    kind_names = ("ItemUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PartUsage(ItemUsage):
    concept = None
    metatype_id = "SysML::Systems::PartUsage"
    metatype_ids = ("SysML::Systems::PartUsage",)
    kind_name = "PartUsage"
    kind_names = ("PartUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConnectionUsage(ConnectorAsUsage, PartUsage):
    concept = None
    metatype_id = "SysML::Systems::ConnectionUsage"
    metatype_ids = ("SysML::Systems::ConnectionUsage",)
    kind_name = "ConnectionUsage"
    kind_names = ("ConnectionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AllocationUsage(ConnectionUsage):
    concept = None
    metatype_id = "SysML::Systems::AllocationUsage"
    metatype_ids = ("SysML::Systems::AllocationUsage",)
    kind_name = "AllocationUsage"
    kind_names = ("AllocationUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Function(Behavior):
    concept = None
    metatype_id = "KerML::Kernel::Function"
    metatype_ids = ("KerML::Kernel::Function",)
    kind_name = "Function"
    kind_names = ("Function",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CalculationDefinition(ActionDefinition, Function):
    concept = None
    metatype_id = "SysML::Systems::CalculationDefinition"
    metatype_ids = ("SysML::Systems::CalculationDefinition",)
    kind_name = "CalculationDefinition"
    kind_names = ("CalculationDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CaseDefinition(CalculationDefinition):
    concept = None
    metatype_id = "SysML::Systems::CaseDefinition"
    metatype_ids = ("SysML::Systems::CaseDefinition",)
    kind_name = "CaseDefinition"
    kind_names = ("CaseDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AnalysisCaseDefinition(CaseDefinition):
    concept = None
    metatype_id = "SysML::Systems::AnalysisCaseDefinition"
    metatype_ids = ("SysML::Systems::AnalysisCaseDefinition",)
    kind_name = "AnalysisCaseDefinition"
    kind_names = ("AnalysisCaseDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Expression(Step):
    concept = None
    metatype_id = "KerML::Kernel::Expression"
    metatype_ids = ("KerML::Kernel::Expression",)
    kind_name = "Expression"
    kind_names = ("Expression",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CalculationUsage(ActionUsage, Expression):
    concept = None
    metatype_id = "SysML::Systems::CalculationUsage"
    metatype_ids = ("SysML::Systems::CalculationUsage",)
    kind_name = "CalculationUsage"
    kind_names = ("CalculationUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CaseUsage(CalculationUsage):
    concept = None
    metatype_id = "SysML::Systems::CaseUsage"
    metatype_ids = ("SysML::Systems::CaseUsage",)
    kind_name = "CaseUsage"
    kind_names = ("CaseUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AnalysisCaseUsage(CaseUsage):
    concept = None
    metatype_id = "SysML::Systems::AnalysisCaseUsage"
    metatype_ids = ("SysML::Systems::AnalysisCaseUsage",)
    kind_name = "AnalysisCaseUsage"
    kind_names = ("AnalysisCaseUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AnnotatingElement(Element):
    concept = None
    metatype_id = "KerML::Root::AnnotatingElement"
    metatype_ids = ("KerML::Root::AnnotatingElement",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Annotation(Relationship):
    concept = None
    metatype_id = "KerML::Root::Annotation"
    metatype_ids = ("KerML::Root::Annotation",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class BooleanExpression(Expression):
    concept = None
    metatype_id = "KerML::Kernel::BooleanExpression"
    metatype_ids = ("KerML::Kernel::BooleanExpression",)
    kind_name = "BooleanExpression"
    kind_names = ("BooleanExpression",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConstraintUsage(BooleanExpression, OccurrenceUsage):
    concept = None
    metatype_id = "SysML::Systems::ConstraintUsage"
    metatype_ids = ("SysML::Systems::ConstraintUsage",)
    kind_name = "ConstraintUsage"
    kind_names = ("ConstraintUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Invariant(BooleanExpression):
    concept = None
    metatype_id = "KerML::Kernel::Invariant"
    metatype_ids = ("KerML::Kernel::Invariant",)
    kind_name = "Invariant"
    kind_names = ("Invariant",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AssertConstraintUsage(ConstraintUsage, Invariant):
    concept = None
    metatype_id = "SysML::Systems::AssertConstraintUsage"
    metatype_ids = ("SysML::Systems::AssertConstraintUsage",)
    kind_name = "AssertConstraintUsage"
    kind_names = ("AssertConstraintUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AssignmentActionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::AssignmentActionUsage"
    metatype_ids = ("SysML::Systems::AssignmentActionUsage",)
    kind_name = "AssignmentActionUsage"
    kind_names = ("AssignmentActionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class DataType(Classifier):
    concept = None
    metatype_id = "KerML::Kernel::DataType"
    metatype_ids = ("KerML::Kernel::DataType",)
    kind_name = "DataType"
    kind_names = ("DataType",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AttributeDefinition(DataType, Definition):
    concept = None
    metatype_id = "SysML::Systems::AttributeDefinition"
    metatype_ids = ("SysML::Systems::AttributeDefinition",)
    kind_name = "AttributeDefinition"
    kind_names = ("AttributeDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class AttributeUsage(Usage):
    concept = None
    metatype_id = "SysML::Systems::AttributeUsage"
    metatype_ids = ("SysML::Systems::AttributeUsage",)
    kind_name = "AttributeUsage"
    kind_names = ("AttributeUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class BindingConnector(Connector):
    concept = None
    metatype_id = "KerML::Kernel::BindingConnector"
    metatype_ids = ("KerML::Kernel::BindingConnector",)
    kind_name = "BindingConnector"
    kind_names = ("BindingConnector",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class BindingConnectorAsUsage(BindingConnector, ConnectorAsUsage):
    concept = None
    metatype_id = "SysML::Systems::BindingConnectorAsUsage"
    metatype_ids = ("SysML::Systems::BindingConnectorAsUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CausationMetadata(ElementView):
    concept = None
    metatype_id = "CauseAndEffect::CausationMetadata"
    metatype_ids = ("CauseAndEffect::CausationMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Metaobject(ElementView):
    concept = None
    metatype_id = "Metaobjects::Metaobject"
    metatype_ids = ("Metaobjects::Metaobject",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SemanticMetadata(Metaobject):
    concept = None
    metatype_id = "Metaobjects::SemanticMetadata"
    metatype_ids = ("Metaobjects::SemanticMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CausationSemanticMetadadata(CausationMetadata, SemanticMetadata):
    concept = None
    metatype_id = "CauseAndEffect::CausationSemanticMetadadata"
    metatype_ids = ("CauseAndEffect::CausationSemanticMetadadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CauseMetadata(SemanticMetadata):
    concept = None
    metatype_id = "CauseAndEffect::CauseMetadata"
    metatype_ids = ("CauseAndEffect::CauseMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class InstantiationExpression(Expression):
    concept = None
    metatype_id = "KerML::Kernel::InstantiationExpression"
    metatype_ids = ("KerML::Kernel::InstantiationExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class InvocationExpression(InstantiationExpression):
    concept = None
    metatype_id = "KerML::Kernel::InvocationExpression"
    metatype_ids = ("KerML::Kernel::InvocationExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class OperatorExpression(InvocationExpression):
    concept = None
    metatype_id = "KerML::Kernel::OperatorExpression"
    metatype_ids = ("KerML::Kernel::OperatorExpression",)
    kind_name = "OperatorExpression"
    kind_names = ("OperatorExpression",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CollectExpression(OperatorExpression):
    concept = None
    metatype_id = "KerML::Kernel::CollectExpression"
    metatype_ids = ("KerML::Kernel::CollectExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Comment(AnnotatingElement):
    concept = None
    metatype_id = "KerML::Root::Comment"
    metatype_ids = ("KerML::Root::Comment",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Predicate(Function):
    concept = None
    metatype_id = "KerML::Kernel::Predicate"
    metatype_ids = ("KerML::Kernel::Predicate",)
    kind_name = "Predicate"
    kind_names = ("Predicate",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConstraintDefinition(OccurrenceDefinition, Predicate):
    concept = None
    metatype_id = "SysML::Systems::ConstraintDefinition"
    metatype_ids = ("SysML::Systems::ConstraintDefinition",)
    kind_name = "ConstraintDefinition"
    kind_names = ("ConstraintDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class RequirementDefinition(ConstraintDefinition):
    concept = None
    metatype_id = "SysML::Systems::RequirementDefinition"
    metatype_ids = ("SysML::Systems::RequirementDefinition",)
    kind_name = "RequirementDefinition"
    kind_names = ("RequirementDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConcernDefinition(RequirementDefinition):
    concept = None
    metatype_id = "SysML::Systems::ConcernDefinition"
    metatype_ids = ("SysML::Systems::ConcernDefinition",)
    kind_name = "ConcernDefinition"
    kind_names = ("ConcernDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class RequirementUsage(ConstraintUsage):
    concept = None
    metatype_id = "SysML::Systems::RequirementUsage"
    metatype_ids = ("SysML::Systems::RequirementUsage",)
    kind_name = "RequirementUsage"
    kind_names = ("RequirementUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConcernUsage(RequirementUsage):
    concept = None
    metatype_id = "SysML::Systems::ConcernUsage"
    metatype_ids = ("SysML::Systems::ConcernUsage",)
    kind_name = "ConcernUsage"
    kind_names = ("ConcernUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PortDefinition(OccurrenceDefinition, Structure):
    concept = None
    metatype_id = "SysML::Systems::PortDefinition"
    metatype_ids = ("SysML::Systems::PortDefinition",)
    kind_name = "PortDefinition"
    kind_names = ("PortDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConjugatedPortDefinition(PortDefinition):
    concept = None
    metatype_id = "SysML::Systems::ConjugatedPortDefinition"
    metatype_ids = ("SysML::Systems::ConjugatedPortDefinition",)
    kind_name = "ConjugatedPortDefinition"
    kind_names = ("ConjugatedPortDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Specialization(Relationship):
    concept = None
    metatype_id = "KerML::Core::Specialization"
    metatype_ids = ("KerML::Core::Specialization",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureTyping(Specialization):
    concept = None
    metatype_id = "KerML::Core::FeatureTyping"
    metatype_ids = ("KerML::Core::FeatureTyping",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConjugatedPortTyping(FeatureTyping):
    concept = None
    metatype_id = "SysML::Systems::ConjugatedPortTyping"
    metatype_ids = ("SysML::Systems::ConjugatedPortTyping",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Conjugation(Relationship):
    concept = None
    metatype_id = "KerML::Core::Conjugation"
    metatype_ids = ("KerML::Core::Conjugation",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ConstructorExpression(InstantiationExpression):
    concept = None
    metatype_id = "KerML::Kernel::ConstructorExpression"
    metatype_ids = ("KerML::Kernel::ConstructorExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ControlNode(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::ControlNode"
    metatype_ids = ("SysML::Systems::ControlNode",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Subsetting(Specialization):
    concept = None
    metatype_id = "KerML::Core::Subsetting"
    metatype_ids = ("KerML::Core::Subsetting",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class CrossSubsetting(Subsetting):
    concept = None
    metatype_id = "KerML::Core::CrossSubsetting"
    metatype_ids = ("KerML::Core::CrossSubsetting",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class DecisionNode(ControlNode):
    concept = None
    metatype_id = "SysML::Systems::DecisionNode"
    metatype_ids = ("SysML::Systems::DecisionNode",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Dependency(Relationship):
    concept = None
    metatype_id = "KerML::Root::Dependency"
    metatype_ids = ("KerML::Root::Dependency",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class DerivationMetadata(SemanticMetadata):
    concept = None
    metatype_id = "RequirementDerivation::DerivationMetadata"
    metatype_ids = ("RequirementDerivation::DerivationMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class DerivedRequirementMetadata(SemanticMetadata):
    concept = None
    metatype_id = "RequirementDerivation::DerivedRequirementMetadata"
    metatype_ids = ("RequirementDerivation::DerivedRequirementMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Differencing(Relationship):
    concept = None
    metatype_id = "KerML::Core::Differencing"
    metatype_ids = ("KerML::Core::Differencing",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Disjoining(Relationship):
    concept = None
    metatype_id = "KerML::Core::Disjoining"
    metatype_ids = ("KerML::Core::Disjoining",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Documentation(Comment):
    concept = None
    metatype_id = "KerML::Root::Documentation"
    metatype_ids = ("KerML::Root::Documentation",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class EffectMetadata(SemanticMetadata):
    concept = None
    metatype_id = "CauseAndEffect::EffectMetadata"
    metatype_ids = ("CauseAndEffect::EffectMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ElementFilterMembership(OwningMembership):
    concept = None
    metatype_id = "KerML::Kernel::ElementFilterMembership"
    metatype_ids = ("KerML::Kernel::ElementFilterMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class EndFeatureMembership(FeatureMembership):
    concept = None
    metatype_id = "KerML::Core::EndFeatureMembership"
    metatype_ids = ("KerML::Core::EndFeatureMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class EnumerationDefinition(AttributeDefinition):
    concept = None
    metatype_id = "SysML::Systems::EnumerationDefinition"
    metatype_ids = ("SysML::Systems::EnumerationDefinition",)
    kind_name = "EnumerationDefinition"
    kind_names = ("EnumerationDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class EnumerationUsage(AttributeUsage):
    concept = None
    metatype_id = "SysML::Systems::EnumerationUsage"
    metatype_ids = ("SysML::Systems::EnumerationUsage",)
    kind_name = "EnumerationUsage"
    kind_names = ("EnumerationUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class EventOccurrenceUsage(OccurrenceUsage):
    concept = None
    metatype_id = "SysML::Systems::EventOccurrenceUsage"
    metatype_ids = ("SysML::Systems::EventOccurrenceUsage",)
    kind_name = "EventOccurrenceUsage"
    kind_names = ("EventOccurrenceUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PerformActionUsage(ActionUsage, EventOccurrenceUsage):
    concept = None
    metatype_id = "SysML::Systems::PerformActionUsage"
    metatype_ids = ("SysML::Systems::PerformActionUsage",)
    kind_name = "PerformActionUsage"
    kind_names = ("PerformActionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class StateUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::StateUsage"
    metatype_ids = ("SysML::Systems::StateUsage",)
    kind_name = "StateUsage"
    kind_names = ("StateUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ExhibitStateUsage(PerformActionUsage, StateUsage):
    concept = None
    metatype_id = "SysML::Systems::ExhibitStateUsage"
    metatype_ids = ("SysML::Systems::ExhibitStateUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Import(Relationship):
    concept = None
    metatype_id = "KerML::Root::Import"
    metatype_ids = ("KerML::Root::Import",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Expose(Import):
    concept = None
    metatype_id = "SysML::Systems::Expose"
    metatype_ids = ("SysML::Systems::Expose",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureChainExpression(OperatorExpression):
    concept = None
    metatype_id = "KerML::Kernel::FeatureChainExpression"
    metatype_ids = ("KerML::Kernel::FeatureChainExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureChaining(Relationship):
    concept = None
    metatype_id = "KerML::Core::FeatureChaining"
    metatype_ids = ("KerML::Core::FeatureChaining",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureInverting(Relationship):
    concept = None
    metatype_id = "KerML::Core::FeatureInverting"
    metatype_ids = ("KerML::Core::FeatureInverting",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureReferenceExpression(Expression):
    concept = None
    metatype_id = "KerML::Kernel::FeatureReferenceExpression"
    metatype_ids = ("KerML::Kernel::FeatureReferenceExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FeatureValue(OwningMembership):
    concept = None
    metatype_id = "KerML::Kernel::FeatureValue"
    metatype_ids = ("KerML::Kernel::FeatureValue",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Flow(Connector, Step):
    concept = None
    metatype_id = "KerML::Kernel::Flow"
    metatype_ids = ("KerML::Kernel::Flow",)
    kind_name = "Flow"
    kind_names = ("Flow",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Interaction(Association, Behavior):
    concept = None
    metatype_id = "KerML::Kernel::Interaction"
    metatype_ids = ("KerML::Kernel::Interaction",)
    kind_name = "Interaction"
    kind_names = ("Interaction",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FlowDefinition(ActionDefinition, Interaction):
    concept = None
    metatype_id = "SysML::Systems::FlowDefinition"
    metatype_ids = ("SysML::Systems::FlowDefinition",)
    kind_name = "FlowDefinition"
    kind_names = ("FlowDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FlowEnd(Feature):
    concept = None
    metatype_id = "KerML::Kernel::FlowEnd"
    metatype_ids = ("KerML::Kernel::FlowEnd",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FlowUsage(ActionUsage, ConnectorAsUsage, Flow):
    concept = None
    metatype_id = "SysML::Systems::FlowUsage"
    metatype_ids = ("SysML::Systems::FlowUsage",)
    kind_name = "FlowUsage"
    kind_names = ("FlowUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LoopActionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::LoopActionUsage"
    metatype_ids = ("SysML::Systems::LoopActionUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ForLoopActionUsage(LoopActionUsage):
    concept = None
    metatype_id = "SysML::Systems::ForLoopActionUsage"
    metatype_ids = ("SysML::Systems::ForLoopActionUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ForkNode(ControlNode):
    concept = None
    metatype_id = "SysML::Systems::ForkNode"
    metatype_ids = ("SysML::Systems::ForkNode",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class RequirementConstraintMembership(FeatureMembership):
    concept = None
    metatype_id = "SysML::Systems::RequirementConstraintMembership"
    metatype_ids = ("SysML::Systems::RequirementConstraintMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class FramedConcernMembership(RequirementConstraintMembership):
    concept = None
    metatype_id = "SysML::Systems::FramedConcernMembership"
    metatype_ids = ("SysML::Systems::FramedConcernMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Icon(ElementView):
    concept = None
    metatype_id = "ImageMetadata::Icon"
    metatype_ids = ("ImageMetadata::Icon",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class IfActionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::IfActionUsage"
    metatype_ids = ("SysML::Systems::IfActionUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class UseCaseUsage(CaseUsage):
    concept = None
    metatype_id = "SysML::Systems::UseCaseUsage"
    metatype_ids = ("SysML::Systems::UseCaseUsage",)
    kind_name = "UseCaseUsage"
    kind_names = ("UseCaseUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class IncludeUseCaseUsage(PerformActionUsage, UseCaseUsage):
    concept = None
    metatype_id = "SysML::Systems::IncludeUseCaseUsage"
    metatype_ids = ("SysML::Systems::IncludeUseCaseUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class IndexExpression(OperatorExpression):
    concept = None
    metatype_id = "KerML::Kernel::IndexExpression"
    metatype_ids = ("KerML::Kernel::IndexExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class InterfaceDefinition(ConnectionDefinition):
    concept = None
    metatype_id = "SysML::Systems::InterfaceDefinition"
    metatype_ids = ("SysML::Systems::InterfaceDefinition",)
    kind_name = "InterfaceDefinition"
    kind_names = ("InterfaceDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class InterfaceUsage(ConnectionUsage):
    concept = None
    metatype_id = "SysML::Systems::InterfaceUsage"
    metatype_ids = ("SysML::Systems::InterfaceUsage",)
    kind_name = "InterfaceUsage"
    kind_names = ("InterfaceUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Intersecting(Relationship):
    concept = None
    metatype_id = "KerML::Core::Intersecting"
    metatype_ids = ("KerML::Core::Intersecting",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Issue(ElementView):
    concept = None
    metatype_id = "ModelingMetadata::Issue"
    metatype_ids = ("ModelingMetadata::Issue",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class JoinNode(ControlNode):
    concept = None
    metatype_id = "SysML::Systems::JoinNode"
    metatype_ids = ("SysML::Systems::JoinNode",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Package(Namespace):
    concept = None
    metatype_id = "KerML::Kernel::Package"
    metatype_ids = ("KerML::Kernel::Package",)
    kind_name = "Package"
    kind_names = ("Package",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LibraryPackage(Package):
    concept = None
    metatype_id = "KerML::Kernel::LibraryPackage"
    metatype_ids = ("KerML::Kernel::LibraryPackage",)
    kind_name = "LibraryPackage"
    kind_names = ("LibraryPackage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LiteralExpression(Expression):
    concept = None
    metatype_id = "KerML::Kernel::LiteralExpression"
    metatype_ids = ("KerML::Kernel::LiteralExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LiteralBoolean(LiteralExpression):
    concept = None
    metatype_id = "KerML::Kernel::LiteralBoolean"
    metatype_ids = ("KerML::Kernel::LiteralBoolean",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LiteralInfinity(LiteralExpression):
    concept = None
    metatype_id = "KerML::Kernel::LiteralInfinity"
    metatype_ids = ("KerML::Kernel::LiteralInfinity",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LiteralInteger(LiteralExpression):
    concept = None
    metatype_id = "KerML::Kernel::LiteralInteger"
    metatype_ids = ("KerML::Kernel::LiteralInteger",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LiteralRational(LiteralExpression):
    concept = None
    metatype_id = "KerML::Kernel::LiteralRational"
    metatype_ids = ("KerML::Kernel::LiteralRational",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class LiteralString(LiteralExpression):
    concept = None
    metatype_id = "KerML::Kernel::LiteralString"
    metatype_ids = ("KerML::Kernel::LiteralString",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MeasureOfEffectiveness(SemanticMetadata):
    concept = None
    metatype_id = "ParametersOfInterestMetadata::MeasureOfEffectiveness"
    metatype_ids = ("ParametersOfInterestMetadata::MeasureOfEffectiveness",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MeasureOfPerformance(SemanticMetadata):
    concept = None
    metatype_id = "ParametersOfInterestMetadata::MeasureOfPerformance"
    metatype_ids = ("ParametersOfInterestMetadata::MeasureOfPerformance",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MembershipImport(Import):
    concept = None
    metatype_id = "KerML::Root::MembershipImport"
    metatype_ids = ("KerML::Root::MembershipImport",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MembershipExpose(Expose, MembershipImport):
    concept = None
    metatype_id = "SysML::Systems::MembershipExpose"
    metatype_ids = ("SysML::Systems::MembershipExpose",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MergeNode(ControlNode):
    concept = None
    metatype_id = "SysML::Systems::MergeNode"
    metatype_ids = ("SysML::Systems::MergeNode",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Metaclass(Structure):
    concept = None
    metatype_id = "KerML::Kernel::Metaclass"
    metatype_ids = ("KerML::Kernel::Metaclass",)
    kind_name = "Metaclass"
    kind_names = ("Metaclass",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MetadataAccessExpression(Expression):
    concept = None
    metatype_id = "KerML::Kernel::MetadataAccessExpression"
    metatype_ids = ("KerML::Kernel::MetadataAccessExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MetadataDefinition(ItemDefinition, Metaclass):
    concept = None
    metatype_id = "SysML::Systems::MetadataDefinition"
    metatype_ids = ("SysML::Systems::MetadataDefinition",)
    kind_name = "MetadataDefinition"
    kind_names = ("MetadataDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MetadataFeature(AnnotatingElement, Feature):
    concept = None
    metatype_id = "KerML::Kernel::MetadataFeature"
    metatype_ids = ("KerML::Kernel::MetadataFeature",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MetadataItem(Metaobject):
    concept = None
    metatype_id = "Metadata::MetadataItem"
    metatype_ids = ("Metadata::MetadataItem",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MetadataUsage(ItemUsage, MetadataFeature):
    concept = None
    metatype_id = "SysML::Systems::MetadataUsage"
    metatype_ids = ("SysML::Systems::MetadataUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MulticausationSemanticMetadata(CausationMetadata, SemanticMetadata):
    concept = None
    metatype_id = "CauseAndEffect::MulticausationSemanticMetadata"
    metatype_ids = ("CauseAndEffect::MulticausationSemanticMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Multiplicity(Feature):
    concept = None
    metatype_id = "KerML::Core::Multiplicity"
    metatype_ids = ("KerML::Core::Multiplicity",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class MultiplicityRange(Multiplicity):
    concept = None
    metatype_id = "KerML::Kernel::MultiplicityRange"
    metatype_ids = ("KerML::Kernel::MultiplicityRange",)
    kind_name = "MultiplicityRange"
    kind_names = ("MultiplicityRange",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class NamespaceImport(Import):
    concept = None
    metatype_id = "KerML::Root::NamespaceImport"
    metatype_ids = ("KerML::Root::NamespaceImport",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class NamespaceExpose(Expose, NamespaceImport):
    concept = None
    metatype_id = "SysML::Systems::NamespaceExpose"
    metatype_ids = ("SysML::Systems::NamespaceExpose",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class NullExpression(Expression):
    concept = None
    metatype_id = "KerML::Kernel::NullExpression"
    metatype_ids = ("KerML::Kernel::NullExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ObjectiveMembership(FeatureMembership):
    concept = None
    metatype_id = "SysML::Systems::ObjectiveMembership"
    metatype_ids = ("SysML::Systems::ObjectiveMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class OriginalRequirementMetadata(SemanticMetadata):
    concept = None
    metatype_id = "RequirementDerivation::OriginalRequirementMetadata"
    metatype_ids = ("RequirementDerivation::OriginalRequirementMetadata",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PayloadFeature(Feature):
    concept = None
    metatype_id = "KerML::Kernel::PayloadFeature"
    metatype_ids = ("KerML::Kernel::PayloadFeature",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PortConjugation(Conjugation):
    concept = None
    metatype_id = "SysML::Systems::PortConjugation"
    metatype_ids = ("SysML::Systems::PortConjugation",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class PortUsage(OccurrenceUsage):
    concept = None
    metatype_id = "SysML::Systems::PortUsage"
    metatype_ids = ("SysML::Systems::PortUsage",)
    kind_name = "PortUsage"
    kind_names = ("PortUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Rationale(ElementView):
    concept = None
    metatype_id = "ModelingMetadata::Rationale"
    metatype_ids = ("ModelingMetadata::Rationale",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Redefinition(Subsetting):
    concept = None
    metatype_id = "KerML::Core::Redefinition"
    metatype_ids = ("KerML::Core::Redefinition",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ReferenceSubsetting(Subsetting):
    concept = None
    metatype_id = "KerML::Core::ReferenceSubsetting"
    metatype_ids = ("KerML::Core::ReferenceSubsetting",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ReferenceUsage(Usage):
    concept = None
    metatype_id = "SysML::Systems::ReferenceUsage"
    metatype_ids = ("SysML::Systems::ReferenceUsage",)
    kind_name = "ReferenceUsage"
    kind_names = ("ReferenceUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Refinement(ElementView):
    concept = None
    metatype_id = "ModelingMetadata::Refinement"
    metatype_ids = ("ModelingMetadata::Refinement",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class RenderingDefinition(PartDefinition):
    concept = None
    metatype_id = "SysML::Systems::RenderingDefinition"
    metatype_ids = ("SysML::Systems::RenderingDefinition",)
    kind_name = "RenderingDefinition"
    kind_names = ("RenderingDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class RenderingUsage(PartUsage):
    concept = None
    metatype_id = "SysML::Systems::RenderingUsage"
    metatype_ids = ("SysML::Systems::RenderingUsage",)
    kind_name = "RenderingUsage"
    kind_names = ("RenderingUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class RequirementVerificationMembership(RequirementConstraintMembership):
    concept = None
    metatype_id = "SysML::Systems::RequirementVerificationMembership"
    metatype_ids = ("SysML::Systems::RequirementVerificationMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ResultExpressionMembership(FeatureMembership):
    concept = None
    metatype_id = "KerML::Kernel::ResultExpressionMembership"
    metatype_ids = ("KerML::Kernel::ResultExpressionMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ReturnParameterMembership(ParameterMembership):
    concept = None
    metatype_id = "KerML::Kernel::ReturnParameterMembership"
    metatype_ids = ("KerML::Kernel::ReturnParameterMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Risk(ElementView):
    concept = None
    metatype_id = "RiskMetadata::Risk"
    metatype_ids = ("RiskMetadata::Risk",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SatisfyRequirementUsage(AssertConstraintUsage, RequirementUsage):
    concept = None
    metatype_id = "SysML::Systems::SatisfyRequirementUsage"
    metatype_ids = ("SysML::Systems::SatisfyRequirementUsage",)
    kind_name = "SatisfyRequirementUsage"
    kind_names = ("SatisfyRequirementUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SelectExpression(OperatorExpression):
    concept = None
    metatype_id = "KerML::Kernel::SelectExpression"
    metatype_ids = ("KerML::Kernel::SelectExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SendActionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::SendActionUsage"
    metatype_ids = ("SysML::Systems::SendActionUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class StakeholderMembership(ParameterMembership):
    concept = None
    metatype_id = "SysML::Systems::StakeholderMembership"
    metatype_ids = ("SysML::Systems::StakeholderMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class StateDefinition(ActionDefinition):
    concept = None
    metatype_id = "SysML::Systems::StateDefinition"
    metatype_ids = ("SysML::Systems::StateDefinition",)
    kind_name = "StateDefinition"
    kind_names = ("StateDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class StateSubactionMembership(FeatureMembership):
    concept = None
    metatype_id = "SysML::Systems::StateSubactionMembership"
    metatype_ids = ("SysML::Systems::StateSubactionMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class StatusInfo(ElementView):
    concept = None
    metatype_id = "ModelingMetadata::StatusInfo"
    metatype_ids = ("ModelingMetadata::StatusInfo",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Subclassification(Specialization):
    concept = None
    metatype_id = "KerML::Core::Subclassification"
    metatype_ids = ("KerML::Core::Subclassification",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SubjectMembership(ParameterMembership):
    concept = None
    metatype_id = "SysML::Systems::SubjectMembership"
    metatype_ids = ("SysML::Systems::SubjectMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Succession(Connector):
    concept = None
    metatype_id = "KerML::Kernel::Succession"
    metatype_ids = ("KerML::Kernel::Succession",)
    kind_name = "Succession"
    kind_names = ("Succession",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SuccessionAsUsage(ConnectorAsUsage, Succession):
    concept = None
    metatype_id = "SysML::Systems::SuccessionAsUsage"
    metatype_ids = ("SysML::Systems::SuccessionAsUsage",)
    kind_name = "SuccessionAsUsage"
    kind_names = ("SuccessionAsUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SuccessionFlow(Flow, Succession):
    concept = None
    metatype_id = "KerML::Kernel::SuccessionFlow"
    metatype_ids = ("KerML::Kernel::SuccessionFlow",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class SuccessionFlowUsage(FlowUsage, SuccessionFlow):
    concept = None
    metatype_id = "SysML::Systems::SuccessionFlowUsage"
    metatype_ids = ("SysML::Systems::SuccessionFlowUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class TerminateActionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::TerminateActionUsage"
    metatype_ids = ("SysML::Systems::TerminateActionUsage",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class TextualRepresentation(AnnotatingElement):
    concept = None
    metatype_id = "KerML::Root::TextualRepresentation"
    metatype_ids = ("KerML::Root::TextualRepresentation",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ToolExecution(ElementView):
    concept = None
    metatype_id = "AnalysisTooling::ToolExecution"
    metatype_ids = ("AnalysisTooling::ToolExecution",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ToolVariable(ElementView):
    concept = None
    metatype_id = "AnalysisTooling::ToolVariable"
    metatype_ids = ("AnalysisTooling::ToolVariable",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class TransitionFeatureMembership(FeatureMembership):
    concept = None
    metatype_id = "SysML::Systems::TransitionFeatureMembership"
    metatype_ids = ("SysML::Systems::TransitionFeatureMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class TransitionUsage(ActionUsage):
    concept = None
    metatype_id = "SysML::Systems::TransitionUsage"
    metatype_ids = ("SysML::Systems::TransitionUsage",)
    kind_name = "TransitionUsage"
    kind_names = ("TransitionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class TriggerInvocationExpression(InvocationExpression):
    concept = None
    metatype_id = "SysML::Systems::TriggerInvocationExpression"
    metatype_ids = ("SysML::Systems::TriggerInvocationExpression",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class TypeFeaturing(Relationship):
    concept = None
    metatype_id = "KerML::Core::TypeFeaturing"
    metatype_ids = ("KerML::Core::TypeFeaturing",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class Unioning(Relationship):
    concept = None
    metatype_id = "KerML::Core::Unioning"
    metatype_ids = ("KerML::Core::Unioning",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class UseCaseDefinition(CaseDefinition):
    concept = None
    metatype_id = "SysML::Systems::UseCaseDefinition"
    metatype_ids = ("SysML::Systems::UseCaseDefinition",)
    kind_name = "UseCaseDefinition"
    kind_names = ("UseCaseDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class VariantMembership(OwningMembership):
    concept = None
    metatype_id = "SysML::Systems::VariantMembership"
    metatype_ids = ("SysML::Systems::VariantMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class VerificationCaseDefinition(CaseDefinition):
    concept = None
    metatype_id = "SysML::Systems::VerificationCaseDefinition"
    metatype_ids = ("SysML::Systems::VerificationCaseDefinition",)
    kind_name = "VerificationCaseDefinition"
    kind_names = ("VerificationCaseDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class VerificationCaseUsage(CaseUsage):
    concept = None
    metatype_id = "SysML::Systems::VerificationCaseUsage"
    metatype_ids = ("SysML::Systems::VerificationCaseUsage",)
    kind_name = "VerificationCaseUsage"
    kind_names = ("VerificationCaseUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class VerificationMethod(ElementView):
    concept = None
    metatype_id = "VerificationCases::VerificationMethod"
    metatype_ids = ("VerificationCases::VerificationMethod",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ViewDefinition(PartDefinition):
    concept = None
    metatype_id = "SysML::Systems::ViewDefinition"
    metatype_ids = ("SysML::Systems::ViewDefinition",)
    kind_name = "ViewDefinition"
    kind_names = ("ViewDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ViewRenderingMembership(FeatureMembership):
    concept = None
    metatype_id = "SysML::Systems::ViewRenderingMembership"
    metatype_ids = ("SysML::Systems::ViewRenderingMembership",)
    kind_name = None
    kind_names = ()

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ViewUsage(PartUsage):
    concept = None
    metatype_id = "SysML::Systems::ViewUsage"
    metatype_ids = ("SysML::Systems::ViewUsage",)
    kind_name = "ViewUsage"
    kind_names = ("ViewUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ViewpointDefinition(RequirementDefinition):
    concept = None
    metatype_id = "SysML::Systems::ViewpointDefinition"
    metatype_ids = ("SysML::Systems::ViewpointDefinition",)
    kind_name = "ViewpointDefinition"
    kind_names = ("ViewpointDefinition",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class ViewpointUsage(RequirementUsage):
    concept = None
    metatype_id = "SysML::Systems::ViewpointUsage"
    metatype_ids = ("SysML::Systems::ViewpointUsage",)
    kind_name = "ViewpointUsage"
    kind_names = ("ViewpointUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


class WhileLoopActionUsage(LoopActionUsage):
    concept = None
    metatype_id = "SysML::Systems::WhileLoopActionUsage"
    metatype_ids = ("SysML::Systems::WhileLoopActionUsage",)
    kind_name = "WhileLoopActionUsage"
    kind_names = ("WhileLoopActionUsage",)

    @property
    def name(self) -> str | None:
        return self.effective_str("name")

    @property
    def declared_name(self) -> str | None:
        return self.effective_str("declared_name")

    @property
    def qualified_name(self) -> str | None:
        return self.effective_str("qualified_name")

    @property
    def documentation(self) -> str | None:
        return self.effective_str("documentation")

    def members(self) -> list[ElementView]:
        return self.references("members")

    def features(self) -> list[ElementView]:
        return self.references("features")

    def owner(self) -> list[ElementView]:
        return self.references("owner")

    def specializes(self) -> list[ElementView]:
        return self.references("specializes")


METAMODEL_CLASSES = (
    Element,
    Namespace,
    Type,
    Feature,
    Usage,
    OccurrenceUsage,
    Step,
    ActionUsage,
    AcceptActionUsage,
    Classifier,
    Class,
    Behavior,
    Definition,
    OccurrenceDefinition,
    ActionDefinition,
    Relationship,
    Membership,
    OwningMembership,
    FeatureMembership,
    ParameterMembership,
    ActorMembership,
    Association,
    Structure,
    AssociationStructure,
    ItemDefinition,
    PartDefinition,
    ConnectionDefinition,
    AllocationDefinition,
    Connector,
    ConnectorAsUsage,
    ItemUsage,
    PartUsage,
    ConnectionUsage,
    AllocationUsage,
    Function,
    CalculationDefinition,
    CaseDefinition,
    AnalysisCaseDefinition,
    Expression,
    CalculationUsage,
    CaseUsage,
    AnalysisCaseUsage,
    AnnotatingElement,
    Annotation,
    BooleanExpression,
    ConstraintUsage,
    Invariant,
    AssertConstraintUsage,
    AssignmentActionUsage,
    DataType,
    AttributeDefinition,
    AttributeUsage,
    BindingConnector,
    BindingConnectorAsUsage,
    CausationMetadata,
    Metaobject,
    SemanticMetadata,
    CausationSemanticMetadadata,
    CauseMetadata,
    InstantiationExpression,
    InvocationExpression,
    OperatorExpression,
    CollectExpression,
    Comment,
    Predicate,
    ConstraintDefinition,
    RequirementDefinition,
    ConcernDefinition,
    RequirementUsage,
    ConcernUsage,
    PortDefinition,
    ConjugatedPortDefinition,
    Specialization,
    FeatureTyping,
    ConjugatedPortTyping,
    Conjugation,
    ConstructorExpression,
    ControlNode,
    Subsetting,
    CrossSubsetting,
    DecisionNode,
    Dependency,
    DerivationMetadata,
    DerivedRequirementMetadata,
    Differencing,
    Disjoining,
    Documentation,
    EffectMetadata,
    ElementFilterMembership,
    EndFeatureMembership,
    EnumerationDefinition,
    EnumerationUsage,
    EventOccurrenceUsage,
    PerformActionUsage,
    StateUsage,
    ExhibitStateUsage,
    Import,
    Expose,
    FeatureChainExpression,
    FeatureChaining,
    FeatureInverting,
    FeatureReferenceExpression,
    FeatureValue,
    Flow,
    Interaction,
    FlowDefinition,
    FlowEnd,
    FlowUsage,
    LoopActionUsage,
    ForLoopActionUsage,
    ForkNode,
    RequirementConstraintMembership,
    FramedConcernMembership,
    Icon,
    IfActionUsage,
    UseCaseUsage,
    IncludeUseCaseUsage,
    IndexExpression,
    InterfaceDefinition,
    InterfaceUsage,
    Intersecting,
    Issue,
    JoinNode,
    Package,
    LibraryPackage,
    LiteralExpression,
    LiteralBoolean,
    LiteralInfinity,
    LiteralInteger,
    LiteralRational,
    LiteralString,
    MeasureOfEffectiveness,
    MeasureOfPerformance,
    MembershipImport,
    MembershipExpose,
    MergeNode,
    Metaclass,
    MetadataAccessExpression,
    MetadataDefinition,
    MetadataFeature,
    MetadataItem,
    MetadataUsage,
    MulticausationSemanticMetadata,
    Multiplicity,
    MultiplicityRange,
    NamespaceImport,
    NamespaceExpose,
    NullExpression,
    ObjectiveMembership,
    OriginalRequirementMetadata,
    PayloadFeature,
    PortConjugation,
    PortUsage,
    Rationale,
    Redefinition,
    ReferenceSubsetting,
    ReferenceUsage,
    Refinement,
    RenderingDefinition,
    RenderingUsage,
    RequirementVerificationMembership,
    ResultExpressionMembership,
    ReturnParameterMembership,
    Risk,
    SatisfyRequirementUsage,
    SelectExpression,
    SendActionUsage,
    StakeholderMembership,
    StateDefinition,
    StateSubactionMembership,
    StatusInfo,
    Subclassification,
    SubjectMembership,
    Succession,
    SuccessionAsUsage,
    SuccessionFlow,
    SuccessionFlowUsage,
    TerminateActionUsage,
    TextualRepresentation,
    ToolExecution,
    ToolVariable,
    TransitionFeatureMembership,
    TransitionUsage,
    TriggerInvocationExpression,
    TypeFeaturing,
    Unioning,
    UseCaseDefinition,
    VariantMembership,
    VerificationCaseDefinition,
    VerificationCaseUsage,
    VerificationMethod,
    ViewDefinition,
    ViewRenderingMembership,
    ViewUsage,
    ViewpointDefinition,
    ViewpointUsage,
    WhileLoopActionUsage,
)

METAMODEL_CLASS_BY_METATYPE = {
    "KerML::Root::Element": Element,
    "KerML::Root::Namespace": Namespace,
    "KerML::Core::Type": Type,
    "KerML::Core::Feature": Feature,
    "SysML::Systems::Usage": Usage,
    "SysML::Systems::OccurrenceUsage": OccurrenceUsage,
    "KerML::Kernel::Step": Step,
    "SysML::Systems::ActionUsage": ActionUsage,
    "SysML::Systems::AcceptActionUsage": AcceptActionUsage,
    "KerML::Core::Classifier": Classifier,
    "KerML::Kernel::Class": Class,
    "KerML::Kernel::Behavior": Behavior,
    "SysML::Systems::Definition": Definition,
    "SysML::Systems::OccurrenceDefinition": OccurrenceDefinition,
    "SysML::Systems::ActionDefinition": ActionDefinition,
    "KerML::Root::Relationship": Relationship,
    "KerML::Root::Membership": Membership,
    "KerML::Root::OwningMembership": OwningMembership,
    "KerML::Core::FeatureMembership": FeatureMembership,
    "KerML::Kernel::ParameterMembership": ParameterMembership,
    "SysML::Systems::ActorMembership": ActorMembership,
    "KerML::Kernel::Association": Association,
    "KerML::Kernel::Structure": Structure,
    "KerML::Kernel::AssociationStructure": AssociationStructure,
    "SysML::Systems::ItemDefinition": ItemDefinition,
    "SysML::Systems::PartDefinition": PartDefinition,
    "SysML::Systems::ConnectionDefinition": ConnectionDefinition,
    "SysML::Systems::AllocationDefinition": AllocationDefinition,
    "KerML::Kernel::Connector": Connector,
    "SysML::Systems::ConnectorAsUsage": ConnectorAsUsage,
    "SysML::Systems::ItemUsage": ItemUsage,
    "SysML::Systems::PartUsage": PartUsage,
    "SysML::Systems::ConnectionUsage": ConnectionUsage,
    "SysML::Systems::AllocationUsage": AllocationUsage,
    "KerML::Kernel::Function": Function,
    "SysML::Systems::CalculationDefinition": CalculationDefinition,
    "SysML::Systems::CaseDefinition": CaseDefinition,
    "SysML::Systems::AnalysisCaseDefinition": AnalysisCaseDefinition,
    "KerML::Kernel::Expression": Expression,
    "SysML::Systems::CalculationUsage": CalculationUsage,
    "SysML::Systems::CaseUsage": CaseUsage,
    "SysML::Systems::AnalysisCaseUsage": AnalysisCaseUsage,
    "KerML::Root::AnnotatingElement": AnnotatingElement,
    "KerML::Root::Annotation": Annotation,
    "KerML::Kernel::BooleanExpression": BooleanExpression,
    "SysML::Systems::ConstraintUsage": ConstraintUsage,
    "KerML::Kernel::Invariant": Invariant,
    "SysML::Systems::AssertConstraintUsage": AssertConstraintUsage,
    "SysML::Systems::AssignmentActionUsage": AssignmentActionUsage,
    "KerML::Kernel::DataType": DataType,
    "SysML::Systems::AttributeDefinition": AttributeDefinition,
    "SysML::Systems::AttributeUsage": AttributeUsage,
    "KerML::Kernel::BindingConnector": BindingConnector,
    "SysML::Systems::BindingConnectorAsUsage": BindingConnectorAsUsage,
    "CauseAndEffect::CausationMetadata": CausationMetadata,
    "Metaobjects::Metaobject": Metaobject,
    "Metaobjects::SemanticMetadata": SemanticMetadata,
    "CauseAndEffect::CausationSemanticMetadadata": CausationSemanticMetadadata,
    "CauseAndEffect::CauseMetadata": CauseMetadata,
    "KerML::Kernel::InstantiationExpression": InstantiationExpression,
    "KerML::Kernel::InvocationExpression": InvocationExpression,
    "KerML::Kernel::OperatorExpression": OperatorExpression,
    "KerML::Kernel::CollectExpression": CollectExpression,
    "KerML::Root::Comment": Comment,
    "KerML::Kernel::Predicate": Predicate,
    "SysML::Systems::ConstraintDefinition": ConstraintDefinition,
    "SysML::Systems::RequirementDefinition": RequirementDefinition,
    "SysML::Systems::ConcernDefinition": ConcernDefinition,
    "SysML::Systems::RequirementUsage": RequirementUsage,
    "SysML::Systems::ConcernUsage": ConcernUsage,
    "SysML::Systems::PortDefinition": PortDefinition,
    "SysML::Systems::ConjugatedPortDefinition": ConjugatedPortDefinition,
    "KerML::Core::Specialization": Specialization,
    "KerML::Core::FeatureTyping": FeatureTyping,
    "SysML::Systems::ConjugatedPortTyping": ConjugatedPortTyping,
    "KerML::Core::Conjugation": Conjugation,
    "KerML::Kernel::ConstructorExpression": ConstructorExpression,
    "SysML::Systems::ControlNode": ControlNode,
    "KerML::Core::Subsetting": Subsetting,
    "KerML::Core::CrossSubsetting": CrossSubsetting,
    "SysML::Systems::DecisionNode": DecisionNode,
    "KerML::Root::Dependency": Dependency,
    "RequirementDerivation::DerivationMetadata": DerivationMetadata,
    "RequirementDerivation::DerivedRequirementMetadata": DerivedRequirementMetadata,
    "KerML::Core::Differencing": Differencing,
    "KerML::Core::Disjoining": Disjoining,
    "KerML::Root::Documentation": Documentation,
    "CauseAndEffect::EffectMetadata": EffectMetadata,
    "KerML::Kernel::ElementFilterMembership": ElementFilterMembership,
    "KerML::Core::EndFeatureMembership": EndFeatureMembership,
    "SysML::Systems::EnumerationDefinition": EnumerationDefinition,
    "SysML::Systems::EnumerationUsage": EnumerationUsage,
    "SysML::Systems::EventOccurrenceUsage": EventOccurrenceUsage,
    "SysML::Systems::PerformActionUsage": PerformActionUsage,
    "SysML::Systems::StateUsage": StateUsage,
    "SysML::Systems::ExhibitStateUsage": ExhibitStateUsage,
    "KerML::Root::Import": Import,
    "SysML::Systems::Expose": Expose,
    "KerML::Kernel::FeatureChainExpression": FeatureChainExpression,
    "KerML::Core::FeatureChaining": FeatureChaining,
    "KerML::Core::FeatureInverting": FeatureInverting,
    "KerML::Kernel::FeatureReferenceExpression": FeatureReferenceExpression,
    "KerML::Kernel::FeatureValue": FeatureValue,
    "KerML::Kernel::Flow": Flow,
    "KerML::Kernel::Interaction": Interaction,
    "SysML::Systems::FlowDefinition": FlowDefinition,
    "KerML::Kernel::FlowEnd": FlowEnd,
    "SysML::Systems::FlowUsage": FlowUsage,
    "SysML::Systems::LoopActionUsage": LoopActionUsage,
    "SysML::Systems::ForLoopActionUsage": ForLoopActionUsage,
    "SysML::Systems::ForkNode": ForkNode,
    "SysML::Systems::RequirementConstraintMembership": RequirementConstraintMembership,
    "SysML::Systems::FramedConcernMembership": FramedConcernMembership,
    "ImageMetadata::Icon": Icon,
    "SysML::Systems::IfActionUsage": IfActionUsage,
    "SysML::Systems::UseCaseUsage": UseCaseUsage,
    "SysML::Systems::IncludeUseCaseUsage": IncludeUseCaseUsage,
    "KerML::Kernel::IndexExpression": IndexExpression,
    "SysML::Systems::InterfaceDefinition": InterfaceDefinition,
    "SysML::Systems::InterfaceUsage": InterfaceUsage,
    "KerML::Core::Intersecting": Intersecting,
    "ModelingMetadata::Issue": Issue,
    "SysML::Systems::JoinNode": JoinNode,
    "KerML::Kernel::Package": Package,
    "KerML::Kernel::LibraryPackage": LibraryPackage,
    "KerML::Kernel::LiteralExpression": LiteralExpression,
    "KerML::Kernel::LiteralBoolean": LiteralBoolean,
    "KerML::Kernel::LiteralInfinity": LiteralInfinity,
    "KerML::Kernel::LiteralInteger": LiteralInteger,
    "KerML::Kernel::LiteralRational": LiteralRational,
    "KerML::Kernel::LiteralString": LiteralString,
    "ParametersOfInterestMetadata::MeasureOfEffectiveness": MeasureOfEffectiveness,
    "ParametersOfInterestMetadata::MeasureOfPerformance": MeasureOfPerformance,
    "KerML::Root::MembershipImport": MembershipImport,
    "SysML::Systems::MembershipExpose": MembershipExpose,
    "SysML::Systems::MergeNode": MergeNode,
    "KerML::Kernel::Metaclass": Metaclass,
    "KerML::Kernel::MetadataAccessExpression": MetadataAccessExpression,
    "SysML::Systems::MetadataDefinition": MetadataDefinition,
    "KerML::Kernel::MetadataFeature": MetadataFeature,
    "Metadata::MetadataItem": MetadataItem,
    "SysML::Systems::MetadataUsage": MetadataUsage,
    "CauseAndEffect::MulticausationSemanticMetadata": MulticausationSemanticMetadata,
    "KerML::Core::Multiplicity": Multiplicity,
    "KerML::Kernel::MultiplicityRange": MultiplicityRange,
    "KerML::Root::NamespaceImport": NamespaceImport,
    "SysML::Systems::NamespaceExpose": NamespaceExpose,
    "KerML::Kernel::NullExpression": NullExpression,
    "SysML::Systems::ObjectiveMembership": ObjectiveMembership,
    "RequirementDerivation::OriginalRequirementMetadata": OriginalRequirementMetadata,
    "KerML::Kernel::PayloadFeature": PayloadFeature,
    "SysML::Systems::PortConjugation": PortConjugation,
    "SysML::Systems::PortUsage": PortUsage,
    "ModelingMetadata::Rationale": Rationale,
    "KerML::Core::Redefinition": Redefinition,
    "KerML::Core::ReferenceSubsetting": ReferenceSubsetting,
    "SysML::Systems::ReferenceUsage": ReferenceUsage,
    "ModelingMetadata::Refinement": Refinement,
    "SysML::Systems::RenderingDefinition": RenderingDefinition,
    "SysML::Systems::RenderingUsage": RenderingUsage,
    "SysML::Systems::RequirementVerificationMembership": RequirementVerificationMembership,
    "KerML::Kernel::ResultExpressionMembership": ResultExpressionMembership,
    "KerML::Kernel::ReturnParameterMembership": ReturnParameterMembership,
    "RiskMetadata::Risk": Risk,
    "SysML::Systems::SatisfyRequirementUsage": SatisfyRequirementUsage,
    "KerML::Kernel::SelectExpression": SelectExpression,
    "SysML::Systems::SendActionUsage": SendActionUsage,
    "SysML::Systems::StakeholderMembership": StakeholderMembership,
    "SysML::Systems::StateDefinition": StateDefinition,
    "SysML::Systems::StateSubactionMembership": StateSubactionMembership,
    "ModelingMetadata::StatusInfo": StatusInfo,
    "KerML::Core::Subclassification": Subclassification,
    "SysML::Systems::SubjectMembership": SubjectMembership,
    "KerML::Kernel::Succession": Succession,
    "SysML::Systems::SuccessionAsUsage": SuccessionAsUsage,
    "KerML::Kernel::SuccessionFlow": SuccessionFlow,
    "SysML::Systems::SuccessionFlowUsage": SuccessionFlowUsage,
    "SysML::Systems::TerminateActionUsage": TerminateActionUsage,
    "KerML::Root::TextualRepresentation": TextualRepresentation,
    "AnalysisTooling::ToolExecution": ToolExecution,
    "AnalysisTooling::ToolVariable": ToolVariable,
    "SysML::Systems::TransitionFeatureMembership": TransitionFeatureMembership,
    "SysML::Systems::TransitionUsage": TransitionUsage,
    "SysML::Systems::TriggerInvocationExpression": TriggerInvocationExpression,
    "KerML::Core::TypeFeaturing": TypeFeaturing,
    "KerML::Core::Unioning": Unioning,
    "SysML::Systems::UseCaseDefinition": UseCaseDefinition,
    "SysML::Systems::VariantMembership": VariantMembership,
    "SysML::Systems::VerificationCaseDefinition": VerificationCaseDefinition,
    "SysML::Systems::VerificationCaseUsage": VerificationCaseUsage,
    "VerificationCases::VerificationMethod": VerificationMethod,
    "SysML::Systems::ViewDefinition": ViewDefinition,
    "SysML::Systems::ViewRenderingMembership": ViewRenderingMembership,
    "SysML::Systems::ViewUsage": ViewUsage,
    "SysML::Systems::ViewpointDefinition": ViewpointDefinition,
    "SysML::Systems::ViewpointUsage": ViewpointUsage,
    "SysML::Systems::WhileLoopActionUsage": WhileLoopActionUsage,
}

METAMODEL_CLASS_BY_KIND = {
    "Feature": Feature,
    "OccurrenceUsage": OccurrenceUsage,
    "Step": Step,
    "ActionUsage": ActionUsage,
    "AcceptActionUsage": AcceptActionUsage,
    "Classifier": Classifier,
    "Class": Class,
    "Behavior": Behavior,
    "ActionDefinition": ActionDefinition,
    "Association": Association,
    "Structure": Structure,
    "AssociationStructure": AssociationStructure,
    "ItemDefinition": ItemDefinition,
    "PartDefinition": PartDefinition,
    "ConnectionDefinition": ConnectionDefinition,
    "AllocationDefinition": AllocationDefinition,
    "Connector": Connector,
    "ItemUsage": ItemUsage,
    "PartUsage": PartUsage,
    "ConnectionUsage": ConnectionUsage,
    "AllocationUsage": AllocationUsage,
    "Function": Function,
    "CalculationDefinition": CalculationDefinition,
    "CaseDefinition": CaseDefinition,
    "AnalysisCaseDefinition": AnalysisCaseDefinition,
    "Expression": Expression,
    "CalculationUsage": CalculationUsage,
    "CaseUsage": CaseUsage,
    "AnalysisCaseUsage": AnalysisCaseUsage,
    "BooleanExpression": BooleanExpression,
    "ConstraintUsage": ConstraintUsage,
    "Invariant": Invariant,
    "AssertConstraintUsage": AssertConstraintUsage,
    "AssignmentActionUsage": AssignmentActionUsage,
    "DataType": DataType,
    "AttributeDefinition": AttributeDefinition,
    "AttributeUsage": AttributeUsage,
    "BindingConnector": BindingConnector,
    "OperatorExpression": OperatorExpression,
    "Predicate": Predicate,
    "ConstraintDefinition": ConstraintDefinition,
    "RequirementDefinition": RequirementDefinition,
    "ConcernDefinition": ConcernDefinition,
    "RequirementUsage": RequirementUsage,
    "ConcernUsage": ConcernUsage,
    "PortDefinition": PortDefinition,
    "ConjugatedPortDefinition": ConjugatedPortDefinition,
    "EnumerationDefinition": EnumerationDefinition,
    "EnumerationUsage": EnumerationUsage,
    "EventOccurrenceUsage": EventOccurrenceUsage,
    "PerformActionUsage": PerformActionUsage,
    "StateUsage": StateUsage,
    "Flow": Flow,
    "Interaction": Interaction,
    "FlowDefinition": FlowDefinition,
    "FlowUsage": FlowUsage,
    "UseCaseUsage": UseCaseUsage,
    "InterfaceDefinition": InterfaceDefinition,
    "InterfaceUsage": InterfaceUsage,
    "Package": Package,
    "LibraryPackage": LibraryPackage,
    "Metaclass": Metaclass,
    "MetadataDefinition": MetadataDefinition,
    "MultiplicityRange": MultiplicityRange,
    "PortUsage": PortUsage,
    "ReferenceUsage": ReferenceUsage,
    "RenderingDefinition": RenderingDefinition,
    "RenderingUsage": RenderingUsage,
    "SatisfyRequirementUsage": SatisfyRequirementUsage,
    "StateDefinition": StateDefinition,
    "Succession": Succession,
    "SuccessionAsUsage": SuccessionAsUsage,
    "TransitionUsage": TransitionUsage,
    "UseCaseDefinition": UseCaseDefinition,
    "VerificationCaseDefinition": VerificationCaseDefinition,
    "VerificationCaseUsage": VerificationCaseUsage,
    "ViewDefinition": ViewDefinition,
    "ViewUsage": ViewUsage,
    "ViewpointDefinition": ViewpointDefinition,
    "ViewpointUsage": ViewpointUsage,
    "WhileLoopActionUsage": WhileLoopActionUsage,
}


def _call_or_value(element: Any, name: str):
    value = getattr(element, name, None)
    return value() if callable(value) else value


def class_for(element: Any) -> type[ElementView]:
    metatype_id = _call_or_value(element, "metatype_id")
    if metatype_id in METAMODEL_CLASS_BY_METATYPE:
        return METAMODEL_CLASS_BY_METATYPE[metatype_id]
    kind = _call_or_value(element, "kind")
    if kind in METAMODEL_CLASS_BY_KIND:
        return METAMODEL_CLASS_BY_KIND[kind]
    return ElementView


def wrap(element: Any) -> ElementView:
    return class_for(element).wrap(element)
