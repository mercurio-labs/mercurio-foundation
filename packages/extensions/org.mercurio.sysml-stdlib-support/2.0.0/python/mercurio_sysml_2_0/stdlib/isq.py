from __future__ import annotations


class ISQNamespace:
    def __init__(self, model):
        self._model = model

    @property
    def AbsoluteActivityValue(self):
        return self._model.element("ISQChemistryMolecular::AbsoluteActivityValue")

    @property
    def AbsorbedDoseRateUnit(self):
        return self._model.element("ISQAtomicNuclear::AbsorbedDoseRateUnit")

    @property
    def AbsorbedDoseRateValue(self):
        return self._model.element("ISQAtomicNuclear::AbsorbedDoseRateValue")

    @property
    def AbsorbedDoseUnit(self):
        return self._model.element("ISQAtomicNuclear::AbsorbedDoseUnit")

    @property
    def AbsorbedDoseValue(self):
        return self._model.element("ISQAtomicNuclear::AbsorbedDoseValue")

    @property
    def AbsorptanceValue(self):
        return self._model.element("ISQLight::AbsorptanceValue")

    @property
    def AbsorptionNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::AbsorptionNumberValue")

    @property
    def AccelerationUnit(self):
        return self._model.element("ISQSpaceTime::AccelerationUnit")

    @property
    def AccelerationValue(self):
        return self._model.element("ISQSpaceTime::AccelerationValue")

    @property
    def AcceptorDensityUnit(self):
        return self._model.element("ISQCondensedMatter::AcceptorDensityUnit")

    @property
    def AcceptorDensityValue(self):
        return self._model.element("ISQCondensedMatter::AcceptorDensityValue")

    @property
    def AcousticImpedanceUnit(self):
        return self._model.element("ISQAcoustics::AcousticImpedanceUnit")

    @property
    def AcousticImpedanceValue(self):
        return self._model.element("ISQAcoustics::AcousticImpedanceValue")

    @property
    def ActionQuantityUnit(self):
        return self._model.element("ISQMechanics::ActionQuantityUnit")

    @property
    def ActionQuantityValue(self):
        return self._model.element("ISQMechanics::ActionQuantityValue")

    @property
    def ActivityCoefficientValue(self):
        return self._model.element("ISQChemistryMolecular::ActivityCoefficientValue")

    @property
    def ActivityDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::ActivityDensityUnit")

    @property
    def ActivityDensityValue(self):
        return self._model.element("ISQAtomicNuclear::ActivityDensityValue")

    @property
    def ActivityFactorValue(self):
        return self._model.element("ISQChemistryMolecular::ActivityFactorValue")

    @property
    def ActivityOfSoluteValue(self):
        return self._model.element("ISQChemistryMolecular::ActivityOfSoluteValue")

    @property
    def ActivityOfSolventValue(self):
        return self._model.element("ISQChemistryMolecular::ActivityOfSolventValue")

    @property
    def AdmittanceUnit(self):
        return self._model.element("ISQElectromagnetism::AdmittanceUnit")

    @property
    def AdmittanceValue(self):
        return self._model.element("ISQElectromagnetism::AdmittanceValue")

    @property
    def AffinityOfAChemicalReactionUnit(self):
        return self._model.element("ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")

    @property
    def AffinityOfAChemicalReactionValue(self):
        return self._model.element("ISQChemistryMolecular::AffinityOfAChemicalReactionValue")

    @property
    def Alfv_nNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::'AlfvénNumberValue'")

    @property
    def AmountOfSubstanceConcentrationUnit(self):
        return self._model.element("ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")

    @property
    def AmountOfSubstanceConcentrationValue(self):
        return self._model.element("ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")

    @property
    def AmountOfSubstanceFractionMoleFractionValue(self):
        return self._model.element("ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue")

    @property
    def AmountOfSubstanceUnit(self):
        return self._model.element("ISQBase::AmountOfSubstanceUnit")

    @property
    def AmountOfSubstanceValue(self):
        return self._model.element("ISQBase::AmountOfSubstanceValue")

    @property
    def Amp_reNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::'AmpèreNumberValue'")

    @property
    def AngularAccelerationUnit(self):
        return self._model.element("ISQSpaceTime::AngularAccelerationUnit")

    @property
    def AngularAccelerationValue(self):
        return self._model.element("ISQSpaceTime::AngularAccelerationValue")

    @property
    def AngularFrequencyUnit(self):
        return self._model.element("ISQSpaceTime::AngularFrequencyUnit")

    @property
    def AngularFrequencyValue(self):
        return self._model.element("ISQSpaceTime::AngularFrequencyValue")

    @property
    def AngularImpulseUnit(self):
        return self._model.element("ISQMechanics::AngularImpulseUnit")

    @property
    def AngularImpulseValue(self):
        return self._model.element("ISQMechanics::AngularImpulseValue")

    @property
    def AngularMeasureUnit(self):
        return self._model.element("ISQSpaceTime::AngularMeasureUnit")

    @property
    def AngularMeasureValue(self):
        return self._model.element("ISQSpaceTime::AngularMeasureValue")

    @property
    def AngularMomentumUnit(self):
        return self._model.element("ISQMechanics::AngularMomentumUnit")

    @property
    def AngularMomentumValue(self):
        return self._model.element("ISQMechanics::AngularMomentumValue")

    @property
    def AngularReciprocalLatticeVectorMagnitudeUnit(self):
        return self._model.element("ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit")

    @property
    def AngularReciprocalLatticeVectorMagnitudeValue(self):
        return self._model.element("ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue")

    @property
    def AngularRepetencyUnit(self):
        return self._model.element("ISQSpaceTime::AngularRepetencyUnit")

    @property
    def AngularRepetencyValue(self):
        return self._model.element("ISQSpaceTime::AngularRepetencyValue")

    @property
    def AngularVelocityUnit(self):
        return self._model.element("ISQSpaceTime::AngularVelocityUnit")

    @property
    def AngularVelocityValue(self):
        return self._model.element("ISQSpaceTime::AngularVelocityValue")

    @property
    def ArchimedesNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ArchimedesNumberValue")

    @property
    def AreaUnit(self):
        return self._model.element("ISQSpaceTime::AreaUnit")

    @property
    def AreaValue(self):
        return self._model.element("ISQSpaceTime::AreaValue")

    @property
    def ArrheniusNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ArrheniusNumberValue")

    @property
    def AtomicAttenuationCoefficientUnit(self):
        return self._model.element("ISQAtomicNuclear::AtomicAttenuationCoefficientUnit")

    @property
    def AtomicAttenuationCoefficientValue(self):
        return self._model.element("ISQAtomicNuclear::AtomicAttenuationCoefficientValue")

    @property
    def AtomicScatteringFactorValue(self):
        return self._model.element("ISQCondensedMatter::AtomicScatteringFactorValue")

    @property
    def AttenuationUnit(self):
        return self._model.element("ISQSpaceTime::AttenuationUnit")

    @property
    def AttenuationValue(self):
        return self._model.element("ISQSpaceTime::AttenuationValue")

    @property
    def AtwoodNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::AtwoodNumberValue")

    @property
    def AverageEnergyLossPerElementaryChargeProducedUnit(self):
        return self._model.element("ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit")

    @property
    def AverageEnergyLossPerElementaryChargeProducedValue(self):
        return self._model.element("ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue")

    @property
    def AverageInformationRateUnit(self):
        return self._model.element("ISQInformation::AverageInformationRateUnit")

    @property
    def AverageInformationRateValue(self):
        return self._model.element("ISQInformation::AverageInformationRateValue")

    @property
    def AverageLogarithmicEnergyDecrementValue(self):
        return self._model.element("ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue")

    @property
    def AverageTransinformationRateUnit(self):
        return self._model.element("ISQInformation::AverageTransinformationRateUnit")

    @property
    def AverageTransinformationRateValue(self):
        return self._model.element("ISQInformation::AverageTransinformationRateValue")

    @property
    def BagnoldNumberForSolidParticlesValue(self):
        return self._model.element("ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue")

    @property
    def BagnoldNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BagnoldNumberValue")

    @property
    def BatchelorNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BatchelorNumberValue")

    @property
    def BejanNumberForEntropyValue(self):
        return self._model.element("ISQCharacteristicNumbers::BejanNumberForEntropyValue")

    @property
    def BejanNumberForHeatTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::BejanNumberForHeatTransferValue")

    @property
    def BejanNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::BejanNumberForMassTransferValue")

    @property
    def BejanNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BejanNumberValue")

    @property
    def BinaryDigitRateUnit(self):
        return self._model.element("ISQInformation::BinaryDigitRateUnit")

    @property
    def BinaryDigitRateValue(self):
        return self._model.element("ISQInformation::BinaryDigitRateValue")

    @property
    def BindingFractionValue(self):
        return self._model.element("ISQAtomicNuclear::BindingFractionValue")

    @property
    def BinghamNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BinghamNumberValue")

    @property
    def BiotNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::BiotNumberForMassTransferValue")

    @property
    def BiotNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BiotNumberValue")

    @property
    def BlakeNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BlakeNumberValue")

    @property
    def BodensteinNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BodensteinNumberValue")

    @property
    def BoltzmannNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BoltzmannNumberValue")

    @property
    def BondNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BondNumberValue")

    @property
    def BrinkmanNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::BrinkmanNumberValue")

    @property
    def CallIntensityUnit(self):
        return self._model.element("ISQInformation::CallIntensityUnit")

    @property
    def CallIntensityValue(self):
        return self._model.element("ISQInformation::CallIntensityValue")

    @property
    def CanonicalPartitionFunctionValue(self):
        return self._model.element("ISQChemistryMolecular::CanonicalPartitionFunctionValue")

    @property
    def CapacitanceUnit(self):
        return self._model.element("ISQElectromagnetism::CapacitanceUnit")

    @property
    def CapacitanceValue(self):
        return self._model.element("ISQElectromagnetism::CapacitanceValue")

    @property
    def CapillaryNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::CapillaryNumberValue")

    @property
    def CarnotNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::CarnotNumberValue")

    @property
    def Cartesian3dMomentOfInertiaMeasurementReference(self):
        return self._model.element("ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference")

    @property
    def Cartesian3dMomentOfInertiaTensor(self):
        return self._model.element("ISQMechanics::Cartesian3dMomentOfInertiaTensor")

    @property
    def Cartesian3dStrainMeasurementReference(self):
        return self._model.element("ISQMechanics::Cartesian3dStrainMeasurementReference")

    @property
    def Cartesian3dStrainTensor(self):
        return self._model.element("ISQMechanics::Cartesian3dStrainTensor")

    @property
    def Cartesian3dStressMeasurementReference(self):
        return self._model.element("ISQMechanics::Cartesian3dStressMeasurementReference")

    @property
    def Cartesian3dStressTensor(self):
        return self._model.element("ISQMechanics::Cartesian3dStressTensor")

    @property
    def CartesianAcceleration3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")

    @property
    def CartesianAcceleration3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianAcceleration3dVector")

    @property
    def CartesianAngularAcceleration3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame")

    @property
    def CartesianAngularAcceleration3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianAngularAcceleration3dVector")

    @property
    def CartesianAngularImpulse3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianAngularImpulse3dCoordinateFrame")

    @property
    def CartesianAngularImpulse3dVector(self):
        return self._model.element("ISQMechanics::CartesianAngularImpulse3dVector")

    @property
    def CartesianAngularMomentum3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianAngularMomentum3dCoordinateFrame")

    @property
    def CartesianAngularMomentum3dVector(self):
        return self._model.element("ISQMechanics::CartesianAngularMomentum3dVector")

    @property
    def CartesianAngularReciprocalLattice3dCoordinateFrame(self):
        return self._model.element("ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame")

    @property
    def CartesianAngularReciprocalLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector")

    @property
    def CartesianAngularVelocity3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame")

    @property
    def CartesianAngularVelocity3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianAngularVelocity3dVector")

    @property
    def CartesianBurgers3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianBurgers3dVector")

    @property
    def CartesianDisplacement3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianDisplacement3dVector")

    @property
    def CartesianDisplacementCurrentDensity3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame")

    @property
    def CartesianDisplacementCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector")

    @property
    def CartesianDragForce3dVector(self):
        return self._model.element("ISQMechanics::CartesianDragForce3dVector")

    @property
    def CartesianElectricCurrentDensity3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame")

    @property
    def CartesianElectricCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricCurrentDensity3dVector")

    @property
    def CartesianElectricDipoleMoment3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame")

    @property
    def CartesianElectricDipoleMoment3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricDipoleMoment3dVector")

    @property
    def CartesianElectricFieldStrength3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame")

    @property
    def CartesianElectricFieldStrength3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricFieldStrength3dVector")

    @property
    def CartesianElectricFluxDensity3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame")

    @property
    def CartesianElectricFluxDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricFluxDensity3dVector")

    @property
    def CartesianElectricPolarization3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame")

    @property
    def CartesianElectricPolarization3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianElectricPolarization3dVector")

    @property
    def CartesianEquilibriumPosition3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianEquilibriumPosition3dVector")

    @property
    def CartesianForce3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianForce3dCoordinateFrame")

    @property
    def CartesianForce3dVector(self):
        return self._model.element("ISQMechanics::CartesianForce3dVector")

    @property
    def CartesianFundamentalLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianFundamentalLattice3dVector")

    @property
    def CartesianFundamentalReciprocalLattice3dCoordinateFrame(self):
        return self._model.element("ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame")

    @property
    def CartesianFundamentalReciprocalLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector")

    @property
    def CartesianImpulse3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianImpulse3dCoordinateFrame")

    @property
    def CartesianImpulse3dVector(self):
        return self._model.element("ISQMechanics::CartesianImpulse3dVector")

    @property
    def CartesianKineticFrictionForce3dVector(self):
        return self._model.element("ISQMechanics::CartesianKineticFrictionForce3dVector")

    @property
    def CartesianLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianLattice3dVector")

    @property
    def CartesianLinearElectricCurrentDensity3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame")

    @property
    def CartesianLinearElectricCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector")

    @property
    def CartesianMagneticDipoleMoment3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame")

    @property
    def CartesianMagneticDipoleMoment3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector")

    @property
    def CartesianMagneticFieldStrength3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame")

    @property
    def CartesianMagneticFieldStrength3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticFieldStrength3dVector")

    @property
    def CartesianMagneticFluxDensity3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame")

    @property
    def CartesianMagneticFluxDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticFluxDensity3dVector")

    @property
    def CartesianMagneticMoment3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame")

    @property
    def CartesianMagneticMoment3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticMoment3dVector")

    @property
    def CartesianMagneticPolarization3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame")

    @property
    def CartesianMagneticPolarization3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticPolarization3dVector")

    @property
    def CartesianMagneticVectorPotential3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame")

    @property
    def CartesianMagneticVectorPotential3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagneticVectorPotential3dVector")

    @property
    def CartesianMagnetization3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame")

    @property
    def CartesianMagnetization3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianMagnetization3dVector")

    @property
    def CartesianMassFlow3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianMassFlow3dCoordinateFrame")

    @property
    def CartesianMassFlow3dVector(self):
        return self._model.element("ISQMechanics::CartesianMassFlow3dVector")

    @property
    def CartesianMomentOfForce3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianMomentOfForce3dCoordinateFrame")

    @property
    def CartesianMomentOfForce3dVector(self):
        return self._model.element("ISQMechanics::CartesianMomentOfForce3dVector")

    @property
    def CartesianMomentum3dCoordinateFrame(self):
        return self._model.element("ISQMechanics::CartesianMomentum3dCoordinateFrame")

    @property
    def CartesianMomentum3dVector(self):
        return self._model.element("ISQMechanics::CartesianMomentum3dVector")

    @property
    def CartesianParticleCurrentDensity3dCoordinateFrame(self):
        return self._model.element("ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame")

    @property
    def CartesianParticleCurrentDensity3dVector(self):
        return self._model.element("ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector")

    @property
    def CartesianParticlePosition3dVector(self):
        return self._model.element("ISQCondensedMatter::CartesianParticlePosition3dVector")

    @property
    def CartesianPosition3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianPosition3dVector")

    @property
    def CartesianPoynting3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianPoynting3dCoordinateFrame")

    @property
    def CartesianPoynting3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianPoynting3dVector")

    @property
    def CartesianRollingResistance3dVector(self):
        return self._model.element("ISQMechanics::CartesianRollingResistance3dVector")

    @property
    def CartesianSoundIntensity3dCoordinateFrame(self):
        return self._model.element("ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")

    @property
    def CartesianSoundIntensity3dVector(self):
        return self._model.element("ISQAcoustics::CartesianSoundIntensity3dVector")

    @property
    def CartesianSoundParticleAcceleration3dVector(self):
        return self._model.element("ISQAcoustics::CartesianSoundParticleAcceleration3dVector")

    @property
    def CartesianSoundParticleDisplacement3dVector(self):
        return self._model.element("ISQAcoustics::CartesianSoundParticleDisplacement3dVector")

    @property
    def CartesianSoundParticleVelocity3dVector(self):
        return self._model.element("ISQAcoustics::CartesianSoundParticleVelocity3dVector")

    @property
    def CartesianSpatial3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CartesianSpatial3dCoordinateFrame")

    @property
    def CartesianSpin3dCoordinateFrame(self):
        return self._model.element("ISQAtomicNuclear::CartesianSpin3dCoordinateFrame")

    @property
    def CartesianSpin3dVector(self):
        return self._model.element("ISQAtomicNuclear::CartesianSpin3dVector")

    @property
    def CartesianStaticFrictionForce3dVector(self):
        return self._model.element("ISQMechanics::CartesianStaticFrictionForce3dVector")

    @property
    def CartesianTotalAngularMomentum3dCoordinateFrame(self):
        return self._model.element("ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame")

    @property
    def CartesianTotalAngularMomentum3dVector(self):
        return self._model.element("ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector")

    @property
    def CartesianTotalCurrentDensity3dCoordinateFrame(self):
        return self._model.element("ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame")

    @property
    def CartesianTotalCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::CartesianTotalCurrentDensity3dVector")

    @property
    def CartesianVelocity3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CartesianVelocity3dCoordinateFrame")

    @property
    def CartesianVelocity3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianVelocity3dVector")

    @property
    def CartesianWave3dVector(self):
        return self._model.element("ISQSpaceTime::CartesianWave3dVector")

    @property
    def CartesianWaveVector3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CartesianWaveVector3dCoordinateFrame")

    @property
    def CartesianWeight3dVector(self):
        return self._model.element("ISQMechanics::CartesianWeight3dVector")

    @property
    def CauchyNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::CauchyNumberValue")

    @property
    def CavitationNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::CavitationNumberValue")

    @property
    def CelsiusTemperatureUnit(self):
        return self._model.element("ISQThermodynamics::CelsiusTemperatureUnit")

    @property
    def CelsiusTemperatureValue(self):
        return self._model.element("ISQThermodynamics::CelsiusTemperatureValue")

    @property
    def ChandrasekharNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ChandrasekharNumberValue")

    @property
    def ChannelCapacityPerCharacterUnit(self):
        return self._model.element("ISQInformation::ChannelCapacityPerCharacterUnit")

    @property
    def ChannelCapacityPerCharacterValue(self):
        return self._model.element("ISQInformation::ChannelCapacityPerCharacterValue")

    @property
    def ChannelTimeCapacityUnit(self):
        return self._model.element("ISQInformation::ChannelTimeCapacityUnit")

    @property
    def ChannelTimeCapacityValue(self):
        return self._model.element("ISQInformation::ChannelTimeCapacityValue")

    @property
    def CharacterMeanEntropyUnit(self):
        return self._model.element("ISQInformation::CharacterMeanEntropyUnit")

    @property
    def CharacterMeanEntropyValue(self):
        return self._model.element("ISQInformation::CharacterMeanEntropyValue")

    @property
    def CharacterMeanTransinformationContentUnit(self):
        return self._model.element("ISQInformation::CharacterMeanTransinformationContentUnit")

    @property
    def CharacterMeanTransinformationContentValue(self):
        return self._model.element("ISQInformation::CharacterMeanTransinformationContentValue")

    @property
    def CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit(self):
        return self._model.element("ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")

    @property
    def CharacteristicImpedanceOfAMediumForLongitudinalWavesValue(self):
        return self._model.element("ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")

    @property
    def ChargeNumberValue(self):
        return self._model.element("ISQAtomicNuclear::ChargeNumberValue")

    @property
    def ChemicalPotentialUnit(self):
        return self._model.element("ISQChemistryMolecular::ChemicalPotentialUnit")

    @property
    def ChemicalPotentialValue(self):
        return self._model.element("ISQChemistryMolecular::ChemicalPotentialValue")

    @property
    def ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue(self):
        return self._model.element("ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue")

    @property
    def ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue(self):
        return self._model.element("ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue")

    @property
    def CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue(self):
        return self._model.element("ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue")

    @property
    def CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue(self):
        return self._model.element("ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue")

    @property
    def ClausiusNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ClausiusNumberValue")

    @property
    def CoefficientOfHeatTransferUnit(self):
        return self._model.element("ISQThermodynamics::CoefficientOfHeatTransferUnit")

    @property
    def CoefficientOfHeatTransferValue(self):
        return self._model.element("ISQThermodynamics::CoefficientOfHeatTransferValue")

    @property
    def CoercivityUnit(self):
        return self._model.element("ISQElectromagnetism::CoercivityUnit")

    @property
    def CoercivityValue(self):
        return self._model.element("ISQElectromagnetism::CoercivityValue")

    @property
    def CompletedCallIntensityUnit(self):
        return self._model.element("ISQInformation::CompletedCallIntensityUnit")

    @property
    def CompletedCallIntensityValue(self):
        return self._model.element("ISQInformation::CompletedCallIntensityValue")

    @property
    def CompressibilityNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::CompressibilityNumberValue")

    @property
    def CompressibilityUnit(self):
        return self._model.element("ISQMechanics::CompressibilityUnit")

    @property
    def CompressibilityValue(self):
        return self._model.element("ISQMechanics::CompressibilityValue")

    @property
    def ConditionalEntropyUnit(self):
        return self._model.element("ISQInformation::ConditionalEntropyUnit")

    @property
    def ConditionalEntropyValue(self):
        return self._model.element("ISQInformation::ConditionalEntropyValue")

    @property
    def ConditionalInformationContentUnit(self):
        return self._model.element("ISQInformation::ConditionalInformationContentUnit")

    @property
    def ConditionalInformationContentValue(self):
        return self._model.element("ISQInformation::ConditionalInformationContentValue")

    @property
    def ConductanceUnit(self):
        return self._model.element("ISQElectromagnetism::ConductanceUnit")

    @property
    def ConductanceValue(self):
        return self._model.element("ISQElectromagnetism::ConductanceValue")

    @property
    def ConductivityUnit(self):
        return self._model.element("ISQElectromagnetism::ConductivityUnit")

    @property
    def ConductivityValue(self):
        return self._model.element("ISQElectromagnetism::ConductivityValue")

    @property
    def CouplingFactorValue(self):
        return self._model.element("ISQElectromagnetism::CouplingFactorValue")

    @property
    def CowlingNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::CowlingNumberValue")

    @property
    def CubicExpansionCoefficientUnit(self):
        return self._model.element("ISQThermodynamics::CubicExpansionCoefficientUnit")

    @property
    def CubicExpansionCoefficientValue(self):
        return self._model.element("ISQThermodynamics::CubicExpansionCoefficientValue")

    @property
    def CurvatureUnit(self):
        return self._model.element("ISQSpaceTime::CurvatureUnit")

    @property
    def CurvatureValue(self):
        return self._model.element("ISQSpaceTime::CurvatureValue")

    @property
    def CylindricalDisplacement3dVector(self):
        return self._model.element("ISQSpaceTime::CylindricalDisplacement3dVector")

    @property
    def CylindricalPosition3dVector(self):
        return self._model.element("ISQSpaceTime::CylindricalPosition3dVector")

    @property
    def CylindricalSpatial3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")

    @property
    def DampingCoefficientUnit(self):
        return self._model.element("ISQSpaceTime::DampingCoefficientUnit")

    @property
    def DampingCoefficientValue(self):
        return self._model.element("ISQSpaceTime::DampingCoefficientValue")

    @property
    def DarcyFrictionFactorValue(self):
        return self._model.element("ISQCharacteristicNumbers::DarcyFrictionFactorValue")

    @property
    def DeanNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::DeanNumberValue")

    @property
    def DeborahNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::DeborahNumberValue")

    @property
    def DebyeWallerFactorValue(self):
        return self._model.element("ISQCondensedMatter::DebyeWallerFactorValue")

    @property
    def DecayConstantUnit(self):
        return self._model.element("ISQAtomicNuclear::DecayConstantUnit")

    @property
    def DecayConstantValue(self):
        return self._model.element("ISQAtomicNuclear::DecayConstantValue")

    @property
    def DecisionContentValue(self):
        return self._model.element("ISQInformation::DecisionContentValue")

    @property
    def DegeneracyValue(self):
        return self._model.element("ISQChemistryMolecular::DegeneracyValue")

    @property
    def DegreeOfDissociationValue(self):
        return self._model.element("ISQChemistryMolecular::DegreeOfDissociationValue")

    @property
    def DensityOfHeatFlowRateUnit(self):
        return self._model.element("ISQThermodynamics::DensityOfHeatFlowRateUnit")

    @property
    def DensityOfHeatFlowRateValue(self):
        return self._model.element("ISQThermodynamics::DensityOfHeatFlowRateValue")

    @property
    def DensityOfVibrationalStatesUnit(self):
        return self._model.element("ISQCondensedMatter::DensityOfVibrationalStatesUnit")

    @property
    def DensityOfVibrationalStatesValue(self):
        return self._model.element("ISQCondensedMatter::DensityOfVibrationalStatesValue")

    @property
    def DiffusionCoefficientUnit(self):
        return self._model.element("ISQChemistryMolecular::DiffusionCoefficientUnit")

    @property
    def DiffusionCoefficientValue(self):
        return self._model.element("ISQChemistryMolecular::DiffusionCoefficientValue")

    @property
    def DirectionAndEnergyDistributionOfCrossSectionUnit(self):
        return self._model.element("ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit")

    @property
    def DirectionAndEnergyDistributionOfCrossSectionValue(self):
        return self._model.element("ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue")

    @property
    def DirectionDistributionOfCrossSectionUnit(self):
        return self._model.element("ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit")

    @property
    def DirectionDistributionOfCrossSectionValue(self):
        return self._model.element("ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue")

    @property
    def Displacement3dVector(self):
        return self._model.element("ISQSpaceTime::Displacement3dVector")

    @property
    def DisplacementCurrentDensityUnit(self):
        return self._model.element("ISQElectromagnetism::DisplacementCurrentDensityUnit")

    @property
    def DisplacementCurrentDensityValue(self):
        return self._model.element("ISQElectromagnetism::DisplacementCurrentDensityValue")

    @property
    def DonorDensityUnit(self):
        return self._model.element("ISQCondensedMatter::DonorDensityUnit")

    @property
    def DonorDensityValue(self):
        return self._model.element("ISQCondensedMatter::DonorDensityValue")

    @property
    def DoseEquivalentUnit(self):
        return self._model.element("ISQAtomicNuclear::DoseEquivalentUnit")

    @property
    def DoseEquivalentValue(self):
        return self._model.element("ISQAtomicNuclear::DoseEquivalentValue")

    @property
    def DragCoefficientValue(self):
        return self._model.element("ISQMechanics::DragCoefficientValue")

    @property
    def DurationUnit(self):
        return self._model.element("ISQBase::DurationUnit")

    @property
    def DurationValue(self):
        return self._model.element("ISQBase::DurationValue")

    @property
    def DynamicCapillaryNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::DynamicCapillaryNumberValue")

    @property
    def DynamicViscosityUnit(self):
        return self._model.element("ISQMechanics::DynamicViscosityUnit")

    @property
    def DynamicViscosityValue(self):
        return self._model.element("ISQMechanics::DynamicViscosityValue")

    @property
    def EckertNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::EckertNumberValue")

    @property
    def EkmanNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::EkmanNumberValue")

    @property
    def ElasticityNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ElasticityNumberValue")

    @property
    def ElectricChargeDensityUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricChargeDensityUnit")

    @property
    def ElectricChargeDensityValue(self):
        return self._model.element("ISQElectromagnetism::ElectricChargeDensityValue")

    @property
    def ElectricChargeUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricChargeUnit")

    @property
    def ElectricChargeValue(self):
        return self._model.element("ISQElectromagnetism::ElectricChargeValue")

    @property
    def ElectricConstantUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricConstantUnit")

    @property
    def ElectricConstantValue(self):
        return self._model.element("ISQElectromagnetism::ElectricConstantValue")

    @property
    def ElectricCurrentDensityUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricCurrentDensityUnit")

    @property
    def ElectricCurrentDensityValue(self):
        return self._model.element("ISQElectromagnetism::ElectricCurrentDensityValue")

    @property
    def ElectricCurrentUnit(self):
        return self._model.element("ISQBase::ElectricCurrentUnit")

    @property
    def ElectricCurrentValue(self):
        return self._model.element("ISQBase::ElectricCurrentValue")

    @property
    def ElectricDipoleMomentUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricDipoleMomentUnit")

    @property
    def ElectricDipoleMomentValue(self):
        return self._model.element("ISQElectromagnetism::ElectricDipoleMomentValue")

    @property
    def ElectricFieldParameterValue(self):
        return self._model.element("ISQCharacteristicNumbers::ElectricFieldParameterValue")

    @property
    def ElectricFieldStrengthUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricFieldStrengthUnit")

    @property
    def ElectricFieldStrengthValue(self):
        return self._model.element("ISQElectromagnetism::ElectricFieldStrengthValue")

    @property
    def ElectricFluxDensityUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricFluxDensityUnit")

    @property
    def ElectricFluxDensityValue(self):
        return self._model.element("ISQElectromagnetism::ElectricFluxDensityValue")

    @property
    def ElectricFluxUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricFluxUnit")

    @property
    def ElectricFluxValue(self):
        return self._model.element("ISQElectromagnetism::ElectricFluxValue")

    @property
    def ElectricPolarizationUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricPolarizationUnit")

    @property
    def ElectricPolarizationValue(self):
        return self._model.element("ISQElectromagnetism::ElectricPolarizationValue")

    @property
    def ElectricPotentialDifferenceUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricPotentialDifferenceUnit")

    @property
    def ElectricPotentialDifferenceValue(self):
        return self._model.element("ISQElectromagnetism::ElectricPotentialDifferenceValue")

    @property
    def ElectricPotentialUnit(self):
        return self._model.element("ISQElectromagnetism::ElectricPotentialUnit")

    @property
    def ElectricPotentialValue(self):
        return self._model.element("ISQElectromagnetism::ElectricPotentialValue")

    @property
    def ElectricSusceptibilityValue(self):
        return self._model.element("ISQElectromagnetism::ElectricSusceptibilityValue")

    @property
    def ElectrolyticConductivityUnit(self):
        return self._model.element("ISQChemistryMolecular::ElectrolyticConductivityUnit")

    @property
    def ElectrolyticConductivityValue(self):
        return self._model.element("ISQChemistryMolecular::ElectrolyticConductivityValue")

    @property
    def ElectromagneticEnergyDensityUnit(self):
        return self._model.element("ISQElectromagnetism::ElectromagneticEnergyDensityUnit")

    @property
    def ElectromagneticEnergyDensityValue(self):
        return self._model.element("ISQElectromagnetism::ElectromagneticEnergyDensityValue")

    @property
    def ElectronDensityUnit(self):
        return self._model.element("ISQCondensedMatter::ElectronDensityUnit")

    @property
    def ElectronDensityValue(self):
        return self._model.element("ISQCondensedMatter::ElectronDensityValue")

    @property
    def EmissivityAtASpecifiedWavelengthValue(self):
        return self._model.element("ISQLight::EmissivityAtASpecifiedWavelengthValue")

    @property
    def EmissivityValue(self):
        return self._model.element("ISQLight::EmissivityValue")

    @property
    def EnergyDensityOfStatesUnit(self):
        return self._model.element("ISQCondensedMatter::EnergyDensityOfStatesUnit")

    @property
    def EnergyDensityOfStatesValue(self):
        return self._model.element("ISQCondensedMatter::EnergyDensityOfStatesValue")

    @property
    def EnergyDistributionOfCrossSectionUnit(self):
        return self._model.element("ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit")

    @property
    def EnergyDistributionOfCrossSectionValue(self):
        return self._model.element("ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue")

    @property
    def EnergyFluenceRateUnit(self):
        return self._model.element("ISQAtomicNuclear::EnergyFluenceRateUnit")

    @property
    def EnergyFluenceRateValue(self):
        return self._model.element("ISQAtomicNuclear::EnergyFluenceRateValue")

    @property
    def EnergyFluenceUnit(self):
        return self._model.element("ISQAtomicNuclear::EnergyFluenceUnit")

    @property
    def EnergyFluenceValue(self):
        return self._model.element("ISQAtomicNuclear::EnergyFluenceValue")

    @property
    def EnergyUnit(self):
        return self._model.element("ISQThermodynamics::EnergyUnit")

    @property
    def EnergyValue(self):
        return self._model.element("ISQThermodynamics::EnergyValue")

    @property
    def EntropyForInformationScienceUnit(self):
        return self._model.element("ISQInformation::EntropyForInformationScienceUnit")

    @property
    def EntropyForInformationScienceValue(self):
        return self._model.element("ISQInformation::EntropyForInformationScienceValue")

    @property
    def EntropyUnit(self):
        return self._model.element("ISQThermodynamics::EntropyUnit")

    @property
    def EntropyValue(self):
        return self._model.element("ISQThermodynamics::EntropyValue")

    @property
    def EquilibriumConstantOnConcentrationBasisUnit(self):
        return self._model.element("ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")

    @property
    def EquilibriumConstantOnConcentrationBasisValue(self):
        return self._model.element("ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")

    @property
    def EquilibriumConstantOnPressureBasisUnit(self):
        return self._model.element("ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")

    @property
    def EquilibriumConstantOnPressureBasisValue(self):
        return self._model.element("ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")

    @property
    def EquivalentBinaryDigitRateUnit(self):
        return self._model.element("ISQInformation::EquivalentBinaryDigitRateUnit")

    @property
    def EquivalentBinaryDigitRateValue(self):
        return self._model.element("ISQInformation::EquivalentBinaryDigitRateValue")

    @property
    def EquivalentBinaryStorageCapacityUnit(self):
        return self._model.element("ISQInformation::EquivalentBinaryStorageCapacityUnit")

    @property
    def EquivalentBinaryStorageCapacityValue(self):
        return self._model.element("ISQInformation::EquivalentBinaryStorageCapacityValue")

    @property
    def EquivocationUnit(self):
        return self._model.element("ISQInformation::EquivocationUnit")

    @property
    def EquivocationValue(self):
        return self._model.element("ISQInformation::EquivocationValue")

    @property
    def ErrorProbabilityValue(self):
        return self._model.element("ISQInformation::ErrorProbabilityValue")

    @property
    def EulerNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::EulerNumberValue")

    @property
    def ExpansionNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ExpansionNumberValue")

    @property
    def ExposureRateUnit(self):
        return self._model.element("ISQAtomicNuclear::ExposureRateUnit")

    @property
    def ExposureRateValue(self):
        return self._model.element("ISQAtomicNuclear::ExposureRateValue")

    @property
    def ExposureUnit(self):
        return self._model.element("ISQAtomicNuclear::ExposureUnit")

    @property
    def ExposureValue(self):
        return self._model.element("ISQAtomicNuclear::ExposureValue")

    @property
    def FanningNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::FanningNumberValue")

    @property
    def FastFissionFactorUnit(self):
        return self._model.element("ISQAtomicNuclear::FastFissionFactorUnit")

    @property
    def FastFissionFactorValue(self):
        return self._model.element("ISQAtomicNuclear::FastFissionFactorValue")

    @property
    def ForceUnit(self):
        return self._model.element("ISQMechanics::ForceUnit")

    @property
    def ForceValue(self):
        return self._model.element("ISQMechanics::ForceValue")

    @property
    def FourierNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::FourierNumberForMassTransferValue")

    @property
    def FourierNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::FourierNumberValue")

    @property
    def FrequencyUnit(self):
        return self._model.element("ISQSpaceTime::FrequencyUnit")

    @property
    def FrequencyValue(self):
        return self._model.element("ISQSpaceTime::FrequencyValue")

    @property
    def FroudeNumberForHeatTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue")

    @property
    def FroudeNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::FroudeNumberValue")

    @property
    def FugacityUnit(self):
        return self._model.element("ISQChemistryMolecular::FugacityUnit")

    @property
    def FugacityValue(self):
        return self._model.element("ISQChemistryMolecular::FugacityValue")

    @property
    def FundamentalReciprocalLatticeVectorMagnitudeUnit(self):
        return self._model.element("ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit")

    @property
    def FundamentalReciprocalLatticeVectorMagnitudeValue(self):
        return self._model.element("ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue")

    @property
    def GFactorOfNucleusOrNuclearParticleValue(self):
        return self._model.element("ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue")

    @property
    def GalileiNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::GalileiNumberValue")

    @property
    def GoertlerNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::GoertlerNumberValue")

    @property
    def Gr_neisenParameterValue(self):
        return self._model.element("ISQCondensedMatter::'GrüneisenParameterValue'")

    @property
    def GraetzNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::GraetzNumberForMassTransferValue")

    @property
    def GraetzNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::GraetzNumberValue")

    @property
    def GrandCanonicalPartitionFunctionValue(self):
        return self._model.element("ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue")

    @property
    def GrashofMagneticNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::GrashofMagneticNumberValue")

    @property
    def GrashofNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::GrashofNumberForMassTransferValue")

    @property
    def GrashofNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::GrashofNumberValue")

    @property
    def GyromagneticRatioOfTheElectronUnit(self):
        return self._model.element("ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit")

    @property
    def GyromagneticRatioOfTheElectronValue(self):
        return self._model.element("ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue")

    @property
    def GyromagneticRatioUnit(self):
        return self._model.element("ISQAtomicNuclear::GyromagneticRatioUnit")

    @property
    def GyromagneticRatioValue(self):
        return self._model.element("ISQAtomicNuclear::GyromagneticRatioValue")

    @property
    def HagenNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::HagenNumberValue")

    @property
    def HallCoefficientUnit(self):
        return self._model.element("ISQCondensedMatter::HallCoefficientUnit")

    @property
    def HallCoefficientValue(self):
        return self._model.element("ISQCondensedMatter::HallCoefficientValue")

    @property
    def HallNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::HallNumberValue")

    @property
    def HartmannNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::HartmannNumberValue")

    @property
    def HartreeEnergyUnit(self):
        return self._model.element("ISQAtomicNuclear::HartreeEnergyUnit")

    @property
    def HartreeEnergyValue(self):
        return self._model.element("ISQAtomicNuclear::HartreeEnergyValue")

    @property
    def HeatCapacityUnit(self):
        return self._model.element("ISQThermodynamics::HeatCapacityUnit")

    @property
    def HeatCapacityValue(self):
        return self._model.element("ISQThermodynamics::HeatCapacityValue")

    @property
    def HeatFlowRateUnit(self):
        return self._model.element("ISQThermodynamics::HeatFlowRateUnit")

    @property
    def HeatFlowRateValue(self):
        return self._model.element("ISQThermodynamics::HeatFlowRateValue")

    @property
    def HeatTransferNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::HeatTransferNumberValue")

    @property
    def Hedstr_mNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::'HedströmNumberValue'")

    @property
    def HoleDensityUnit(self):
        return self._model.element("ISQCondensedMatter::HoleDensityUnit")

    @property
    def HoleDensityValue(self):
        return self._model.element("ISQCondensedMatter::HoleDensityValue")

    @property
    def HookeNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::HookeNumberValue")

    @property
    def I(self):
        return self._model.element("ISQBase::'International System of Quantities'::I")

    @property
    def ISQ(self):
        return self._model.element("ISQ")

    @property
    def ISQAcoustics(self):
        return self._model.element("ISQAcoustics")

    @property
    def ISQAtomicNuclear(self):
        return self._model.element("ISQAtomicNuclear")

    @property
    def ISQBase(self):
        return self._model.element("ISQBase")

    @property
    def ISQCharacteristicNumbers(self):
        return self._model.element("ISQCharacteristicNumbers")

    @property
    def ISQChemistryMolecular(self):
        return self._model.element("ISQChemistryMolecular")

    @property
    def ISQCondensedMatter(self):
        return self._model.element("ISQCondensedMatter")

    @property
    def ISQElectromagnetism(self):
        return self._model.element("ISQElectromagnetism")

    @property
    def ISQInformation(self):
        return self._model.element("ISQInformation")

    @property
    def ISQLight(self):
        return self._model.element("ISQLight")

    @property
    def ISQMechanics(self):
        return self._model.element("ISQMechanics")

    @property
    def ISQSpaceTime(self):
        return self._model.element("ISQSpaceTime")

    @property
    def ISQThermodynamics(self):
        return self._model.element("ISQThermodynamics")

    @property
    def IlluminanceUnit(self):
        return self._model.element("ISQLight::IlluminanceUnit")

    @property
    def IlluminanceValue(self):
        return self._model.element("ISQLight::IlluminanceValue")

    @property
    def ImpedanceUnit(self):
        return self._model.element("ISQElectromagnetism::ImpedanceUnit")

    @property
    def ImpedanceValue(self):
        return self._model.element("ISQElectromagnetism::ImpedanceValue")

    @property
    def ImpulseUnit(self):
        return self._model.element("ISQMechanics::ImpulseUnit")

    @property
    def ImpulseValue(self):
        return self._model.element("ISQMechanics::ImpulseValue")

    @property
    def InductanceUnit(self):
        return self._model.element("ISQElectromagnetism::InductanceUnit")

    @property
    def InductanceValue(self):
        return self._model.element("ISQElectromagnetism::InductanceValue")

    @property
    def InfiniteMultiplicationFactorUnit(self):
        return self._model.element("ISQAtomicNuclear::InfiniteMultiplicationFactorUnit")

    @property
    def InfiniteMultiplicationFactorValue(self):
        return self._model.element("ISQAtomicNuclear::InfiniteMultiplicationFactorValue")

    @property
    def InformationContentUnit(self):
        return self._model.element("ISQInformation::InformationContentUnit")

    @property
    def InformationContentValue(self):
        return self._model.element("ISQInformation::InformationContentValue")

    @property
    def InternalConversionFactorValue(self):
        return self._model.element("ISQAtomicNuclear::InternalConversionFactorValue")

    @property
    def International_System_of_Quantities(self):
        return self._model.element("ISQBase::'International System of Quantities'")

    @property
    def IntrinsicCarrierDensityUnit(self):
        return self._model.element("ISQCondensedMatter::IntrinsicCarrierDensityUnit")

    @property
    def IntrinsicCarrierDensityValue(self):
        return self._model.element("ISQCondensedMatter::IntrinsicCarrierDensityValue")

    @property
    def IonNumberDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::IonNumberDensityUnit")

    @property
    def IonNumberDensityValue(self):
        return self._model.element("ISQAtomicNuclear::IonNumberDensityValue")

    @property
    def IonicStrengthUnit(self):
        return self._model.element("ISQChemistryMolecular::IonicStrengthUnit")

    @property
    def IonicStrengthValue(self):
        return self._model.element("ISQChemistryMolecular::IonicStrengthValue")

    @property
    def IrradianceUnit(self):
        return self._model.element("ISQLight::IrradianceUnit")

    @property
    def IrradianceValue(self):
        return self._model.element("ISQLight::IrradianceValue")

    @property
    def IrrelevanceUnit(self):
        return self._model.element("ISQInformation::IrrelevanceUnit")

    @property
    def IrrelevanceValue(self):
        return self._model.element("ISQInformation::IrrelevanceValue")

    @property
    def IsentropicCompressibilityUnit(self):
        return self._model.element("ISQThermodynamics::IsentropicCompressibilityUnit")

    @property
    def IsentropicCompressibilityValue(self):
        return self._model.element("ISQThermodynamics::IsentropicCompressibilityValue")

    @property
    def IsentropicExponentValue(self):
        return self._model.element("ISQThermodynamics::IsentropicExponentValue")

    @property
    def IsothermalCompressibilityUnit(self):
        return self._model.element("ISQThermodynamics::IsothermalCompressibilityUnit")

    @property
    def IsothermalCompressibilityValue(self):
        return self._model.element("ISQThermodynamics::IsothermalCompressibilityValue")

    @property
    def J(self):
        return self._model.element("ISQBase::'International System of Quantities'::J")

    @property
    def JFactorValue(self):
        return self._model.element("ISQCharacteristicNumbers::JFactorValue")

    @property
    def JointInformationContentUnit(self):
        return self._model.element("ISQInformation::JointInformationContentUnit")

    @property
    def JointInformationContentValue(self):
        return self._model.element("ISQInformation::JointInformationContentValue")

    @property
    def JouleMagneticNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::JouleMagneticNumberValue")

    @property
    def JouleThomsonCoefficientUnit(self):
        return self._model.element("ISQThermodynamics::JouleThomsonCoefficientUnit")

    @property
    def JouleThomsonCoefficientValue(self):
        return self._model.element("ISQThermodynamics::JouleThomsonCoefficientValue")

    @property
    def KermaRateUnit(self):
        return self._model.element("ISQAtomicNuclear::KermaRateUnit")

    @property
    def KermaRateValue(self):
        return self._model.element("ISQAtomicNuclear::KermaRateValue")

    @property
    def KermaUnit(self):
        return self._model.element("ISQAtomicNuclear::KermaUnit")

    @property
    def KermaValue(self):
        return self._model.element("ISQAtomicNuclear::KermaValue")

    @property
    def KinematicViscosityUnit(self):
        return self._model.element("ISQMechanics::KinematicViscosityUnit")

    @property
    def KinematicViscosityValue(self):
        return self._model.element("ISQMechanics::KinematicViscosityValue")

    @property
    def KineticFrictionFactorValue(self):
        return self._model.element("ISQMechanics::KineticFrictionFactorValue")

    @property
    def KnudsenNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::KnudsenNumberValue")

    @property
    def L(self):
        return self._model.element("ISQBase::'International System of Quantities'::L")

    @property
    def LagrangeNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LagrangeNumberValue")

    @property
    def LandauGinzburgNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LandauGinzburgNumberValue")

    @property
    def LandeFactorValue(self):
        return self._model.element("ISQAtomicNuclear::LandeFactorValue")

    @property
    def LaplaceNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LaplaceNumberValue")

    @property
    def LarmorFrequencyUnit(self):
        return self._model.element("ISQAtomicNuclear::LarmorFrequencyUnit")

    @property
    def LarmorFrequencyValue(self):
        return self._model.element("ISQAtomicNuclear::LarmorFrequencyValue")

    @property
    def LavalNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LavalNumberValue")

    @property
    def LeakageFactorValue(self):
        return self._model.element("ISQElectromagnetism::LeakageFactorValue")

    @property
    def LengthUnit(self):
        return self._model.element("ISQBase::LengthUnit")

    @property
    def LengthValue(self):
        return self._model.element("ISQBase::LengthValue")

    @property
    def LethargyValue(self):
        return self._model.element("ISQAtomicNuclear::LethargyValue")

    @property
    def LewisNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LewisNumberValue")

    @property
    def LiftCoefficientValue(self):
        return self._model.element("ISQCharacteristicNumbers::LiftCoefficientValue")

    @property
    def LinearAbsorptionCoefficientUnit(self):
        return self._model.element("ISQLight::LinearAbsorptionCoefficientUnit")

    @property
    def LinearAbsorptionCoefficientValue(self):
        return self._model.element("ISQLight::LinearAbsorptionCoefficientValue")

    @property
    def LinearAttenuationCoefficientForIonizingRadiationUnit(self):
        return self._model.element("ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit")

    @property
    def LinearAttenuationCoefficientForIonizingRadiationValue(self):
        return self._model.element("ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue")

    @property
    def LinearAttenuationCoefficientUnit(self):
        return self._model.element("ISQLight::LinearAttenuationCoefficientUnit")

    @property
    def LinearAttenuationCoefficientValue(self):
        return self._model.element("ISQLight::LinearAttenuationCoefficientValue")

    @property
    def LinearDensityOfElectricChargeUnit(self):
        return self._model.element("ISQElectromagnetism::LinearDensityOfElectricChargeUnit")

    @property
    def LinearDensityOfElectricChargeValue(self):
        return self._model.element("ISQElectromagnetism::LinearDensityOfElectricChargeValue")

    @property
    def LinearElectricCurrentDensityUnit(self):
        return self._model.element("ISQElectromagnetism::LinearElectricCurrentDensityUnit")

    @property
    def LinearElectricCurrentDensityValue(self):
        return self._model.element("ISQElectromagnetism::LinearElectricCurrentDensityValue")

    @property
    def LinearEnergyTransferUnit(self):
        return self._model.element("ISQAtomicNuclear::LinearEnergyTransferUnit")

    @property
    def LinearEnergyTransferValue(self):
        return self._model.element("ISQAtomicNuclear::LinearEnergyTransferValue")

    @property
    def LinearExpansionCoefficientUnit(self):
        return self._model.element("ISQThermodynamics::LinearExpansionCoefficientUnit")

    @property
    def LinearExpansionCoefficientValue(self):
        return self._model.element("ISQThermodynamics::LinearExpansionCoefficientValue")

    @property
    def LinearIonizationUnit(self):
        return self._model.element("ISQAtomicNuclear::LinearIonizationUnit")

    @property
    def LinearIonizationValue(self):
        return self._model.element("ISQAtomicNuclear::LinearIonizationValue")

    @property
    def LinearMassDensityUnit(self):
        return self._model.element("ISQMechanics::LinearMassDensityUnit")

    @property
    def LinearMassDensityValue(self):
        return self._model.element("ISQMechanics::LinearMassDensityValue")

    @property
    def LinkedFluxUnit(self):
        return self._model.element("ISQElectromagnetism::LinkedFluxUnit")

    @property
    def LinkedFluxValue(self):
        return self._model.element("ISQElectromagnetism::LinkedFluxValue")

    @property
    def LockhartMartinelliParameterValue(self):
        return self._model.element("ISQCharacteristicNumbers::LockhartMartinelliParameterValue")

    @property
    def LogarithmicDecrementValue(self):
        return self._model.element("ISQSpaceTime::LogarithmicDecrementValue")

    @property
    def LogarithmicFrequencyRangeUnit(self):
        return self._model.element("ISQAcoustics::LogarithmicFrequencyRangeUnit")

    @property
    def LogarithmicFrequencyRangeValue(self):
        return self._model.element("ISQAcoustics::LogarithmicFrequencyRangeValue")

    @property
    def LongRangeOrderParameterValue(self):
        return self._model.element("ISQCondensedMatter::LongRangeOrderParameterValue")

    @property
    def LorentzNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LorentzNumberValue")

    @property
    def LorenzCoefficientUnit(self):
        return self._model.element("ISQCondensedMatter::LorenzCoefficientUnit")

    @property
    def LorenzCoefficientValue(self):
        return self._model.element("ISQCondensedMatter::LorenzCoefficientValue")

    @property
    def LossFactorValue(self):
        return self._model.element("ISQElectromagnetism::LossFactorValue")

    @property
    def LossProbabilityValue(self):
        return self._model.element("ISQInformation::LossProbabilityValue")

    @property
    def LuminanceFactorValue(self):
        return self._model.element("ISQLight::LuminanceFactorValue")

    @property
    def LuminanceUnit(self):
        return self._model.element("ISQLight::LuminanceUnit")

    @property
    def LuminanceValue(self):
        return self._model.element("ISQLight::LuminanceValue")

    @property
    def LuminousAbsorptanceValue(self):
        return self._model.element("ISQLight::LuminousAbsorptanceValue")

    @property
    def LuminousEfficacyOfASourceUnit(self):
        return self._model.element("ISQLight::LuminousEfficacyOfASourceUnit")

    @property
    def LuminousEfficacyOfASourceValue(self):
        return self._model.element("ISQLight::LuminousEfficacyOfASourceValue")

    @property
    def LuminousEfficacyOfRadiationUnit(self):
        return self._model.element("ISQLight::LuminousEfficacyOfRadiationUnit")

    @property
    def LuminousEfficacyOfRadiationValue(self):
        return self._model.element("ISQLight::LuminousEfficacyOfRadiationValue")

    @property
    def LuminousEfficiencyValue(self):
        return self._model.element("ISQLight::LuminousEfficiencyValue")

    @property
    def LuminousEnergyUnit(self):
        return self._model.element("ISQLight::LuminousEnergyUnit")

    @property
    def LuminousEnergyValue(self):
        return self._model.element("ISQLight::LuminousEnergyValue")

    @property
    def LuminousExitanceUnit(self):
        return self._model.element("ISQLight::LuminousExitanceUnit")

    @property
    def LuminousExitanceValue(self):
        return self._model.element("ISQLight::LuminousExitanceValue")

    @property
    def LuminousExposureUnit(self):
        return self._model.element("ISQLight::LuminousExposureUnit")

    @property
    def LuminousExposureValue(self):
        return self._model.element("ISQLight::LuminousExposureValue")

    @property
    def LuminousFluxUnit(self):
        return self._model.element("ISQLight::LuminousFluxUnit")

    @property
    def LuminousFluxValue(self):
        return self._model.element("ISQLight::LuminousFluxValue")

    @property
    def LuminousIntensityUnit(self):
        return self._model.element("ISQBase::LuminousIntensityUnit")

    @property
    def LuminousIntensityValue(self):
        return self._model.element("ISQBase::LuminousIntensityValue")

    @property
    def LuminousReflectanceValue(self):
        return self._model.element("ISQLight::LuminousReflectanceValue")

    @property
    def LuminousTransmittanceValue(self):
        return self._model.element("ISQLight::LuminousTransmittanceValue")

    @property
    def LundquistNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::LundquistNumberValue")

    @property
    def M(self):
        return self._model.element("ISQBase::'International System of Quantities'::M")

    @property
    def MachNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::MachNumberValue")

    @property
    def MagneticConstantUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticConstantUnit")

    @property
    def MagneticConstantValue(self):
        return self._model.element("ISQElectromagnetism::MagneticConstantValue")

    @property
    def MagneticDipoleMomentUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticDipoleMomentUnit")

    @property
    def MagneticDipoleMomentValue(self):
        return self._model.element("ISQElectromagnetism::MagneticDipoleMomentValue")

    @property
    def MagneticFieldStrengthUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticFieldStrengthUnit")

    @property
    def MagneticFieldStrengthValue(self):
        return self._model.element("ISQElectromagnetism::MagneticFieldStrengthValue")

    @property
    def MagneticFluxDensityUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticFluxDensityUnit")

    @property
    def MagneticFluxDensityValue(self):
        return self._model.element("ISQElectromagnetism::MagneticFluxDensityValue")

    @property
    def MagneticFluxUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticFluxUnit")

    @property
    def MagneticFluxValue(self):
        return self._model.element("ISQElectromagnetism::MagneticFluxValue")

    @property
    def MagneticMomentUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticMomentUnit")

    @property
    def MagneticMomentValue(self):
        return self._model.element("ISQElectromagnetism::MagneticMomentValue")

    @property
    def MagneticNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::MagneticNumberValue")

    @property
    def MagneticPolarizationUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticPolarizationUnit")

    @property
    def MagneticPolarizationValue(self):
        return self._model.element("ISQElectromagnetism::MagneticPolarizationValue")

    @property
    def MagneticPressureNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::MagneticPressureNumberValue")

    @property
    def MagneticSusceptibilityValue(self):
        return self._model.element("ISQElectromagnetism::MagneticSusceptibilityValue")

    @property
    def MagneticVectorPotentialUnit(self):
        return self._model.element("ISQElectromagnetism::MagneticVectorPotentialUnit")

    @property
    def MagneticVectorPotentialValue(self):
        return self._model.element("ISQElectromagnetism::MagneticVectorPotentialValue")

    @property
    def MagnetizationUnit(self):
        return self._model.element("ISQElectromagnetism::MagnetizationUnit")

    @property
    def MagnetizationValue(self):
        return self._model.element("ISQElectromagnetism::MagnetizationValue")

    @property
    def MagnetomotiveForceUnit(self):
        return self._model.element("ISQElectromagnetism::MagnetomotiveForceUnit")

    @property
    def MagnetomotiveForceValue(self):
        return self._model.element("ISQElectromagnetism::MagnetomotiveForceValue")

    @property
    def MarangoniNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::MarangoniNumberValue")

    @property
    def MassAbsorptionCoefficientUnit(self):
        return self._model.element("ISQLight::MassAbsorptionCoefficientUnit")

    @property
    def MassAbsorptionCoefficientValue(self):
        return self._model.element("ISQLight::MassAbsorptionCoefficientValue")

    @property
    def MassAttenuationCoefficientForIonizingRadiationUnit(self):
        return self._model.element("ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit")

    @property
    def MassAttenuationCoefficientForIonizingRadiationValue(self):
        return self._model.element("ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue")

    @property
    def MassAttenuationCoefficientUnit(self):
        return self._model.element("ISQLight::MassAttenuationCoefficientUnit")

    @property
    def MassAttenuationCoefficientValue(self):
        return self._model.element("ISQLight::MassAttenuationCoefficientValue")

    @property
    def MassChangeRateUnit(self):
        return self._model.element("ISQMechanics::MassChangeRateUnit")

    @property
    def MassChangeRateValue(self):
        return self._model.element("ISQMechanics::MassChangeRateValue")

    @property
    def MassConcentrationOfWaterUnit(self):
        return self._model.element("ISQThermodynamics::MassConcentrationOfWaterUnit")

    @property
    def MassConcentrationOfWaterValue(self):
        return self._model.element("ISQThermodynamics::MassConcentrationOfWaterValue")

    @property
    def MassConcentrationOfWaterVapourAbsoluteHumidityUnit(self):
        return self._model.element("ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit")

    @property
    def MassConcentrationOfWaterVapourAbsoluteHumidityValue(self):
        return self._model.element("ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue")

    @property
    def MassConcentrationUnit(self):
        return self._model.element("ISQChemistryMolecular::MassConcentrationUnit")

    @property
    def MassConcentrationValue(self):
        return self._model.element("ISQChemistryMolecular::MassConcentrationValue")

    @property
    def MassDensityUnit(self):
        return self._model.element("ISQMechanics::MassDensityUnit")

    @property
    def MassDensityValue(self):
        return self._model.element("ISQMechanics::MassDensityValue")

    @property
    def MassEnergyTransferCoefficientUnit(self):
        return self._model.element("ISQAtomicNuclear::MassEnergyTransferCoefficientUnit")

    @property
    def MassEnergyTransferCoefficientValue(self):
        return self._model.element("ISQAtomicNuclear::MassEnergyTransferCoefficientValue")

    @property
    def MassFlowRateUnit(self):
        return self._model.element("ISQMechanics::MassFlowRateUnit")

    @property
    def MassFlowRateValue(self):
        return self._model.element("ISQMechanics::MassFlowRateValue")

    @property
    def MassFlowUnit(self):
        return self._model.element("ISQMechanics::MassFlowUnit")

    @property
    def MassFlowValue(self):
        return self._model.element("ISQMechanics::MassFlowValue")

    @property
    def MassFractionOfDryMatterValue(self):
        return self._model.element("ISQThermodynamics::MassFractionOfDryMatterValue")

    @property
    def MassFractionOfWaterValue(self):
        return self._model.element("ISQThermodynamics::MassFractionOfWaterValue")

    @property
    def MassFractionValue(self):
        return self._model.element("ISQChemistryMolecular::MassFractionValue")

    @property
    def MassRatioOfWaterToDryMatterValue(self):
        return self._model.element("ISQThermodynamics::MassRatioOfWaterToDryMatterValue")

    @property
    def MassRatioOfWaterVapourToDryGasValue(self):
        return self._model.element("ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue")

    @property
    def MassTransferFactorValue(self):
        return self._model.element("ISQCharacteristicNumbers::MassTransferFactorValue")

    @property
    def MassUnit(self):
        return self._model.element("ISQBase::MassUnit")

    @property
    def MassValue(self):
        return self._model.element("ISQBase::MassValue")

    @property
    def MassieuFunctionUnit(self):
        return self._model.element("ISQThermodynamics::MassieuFunctionUnit")

    @property
    def MassieuFunctionValue(self):
        return self._model.element("ISQThermodynamics::MassieuFunctionValue")

    @property
    def MaximumEntropyUnit(self):
        return self._model.element("ISQInformation::MaximumEntropyUnit")

    @property
    def MaximumEntropyValue(self):
        return self._model.element("ISQInformation::MaximumEntropyValue")

    @property
    def MaximumLuminousEfficacyUnit(self):
        return self._model.element("ISQLight::MaximumLuminousEfficacyUnit")

    @property
    def MaximumLuminousEfficacyValue(self):
        return self._model.element("ISQLight::MaximumLuminousEfficacyValue")

    @property
    def MaximumThermalEfficiencyValue(self):
        return self._model.element("ISQThermodynamics::MaximumThermalEfficiencyValue")

    @property
    def MeanMassRangeUnit(self):
        return self._model.element("ISQAtomicNuclear::MeanMassRangeUnit")

    @property
    def MeanMassRangeValue(self):
        return self._model.element("ISQAtomicNuclear::MeanMassRangeValue")

    @property
    def MeanQueueLengthValue(self):
        return self._model.element("ISQInformation::MeanQueueLengthValue")

    @property
    def MeanTransinformationContentUnit(self):
        return self._model.element("ISQInformation::MeanTransinformationContentUnit")

    @property
    def MeanTransinformationContentValue(self):
        return self._model.element("ISQInformation::MeanTransinformationContentValue")

    @property
    def MechanicalEfficiencyValue(self):
        return self._model.element("ISQMechanics::MechanicalEfficiencyValue")

    @property
    def MobilityRatioValue(self):
        return self._model.element("ISQCondensedMatter::MobilityRatioValue")

    @property
    def MobilityUnit(self):
        return self._model.element("ISQAtomicNuclear::MobilityUnit")

    @property
    def MobilityValue(self):
        return self._model.element("ISQAtomicNuclear::MobilityValue")

    @property
    def ModulationRateUnit(self):
        return self._model.element("ISQInformation::ModulationRateUnit")

    @property
    def ModulationRateValue(self):
        return self._model.element("ISQInformation::ModulationRateValue")

    @property
    def ModulusOfAdmittanceUnit(self):
        return self._model.element("ISQElectromagnetism::ModulusOfAdmittanceUnit")

    @property
    def ModulusOfAdmittanceValue(self):
        return self._model.element("ISQElectromagnetism::ModulusOfAdmittanceValue")

    @property
    def ModulusOfCompressionUnit(self):
        return self._model.element("ISQMechanics::ModulusOfCompressionUnit")

    @property
    def ModulusOfCompressionValue(self):
        return self._model.element("ISQMechanics::ModulusOfCompressionValue")

    @property
    def ModulusOfElasticityUnit(self):
        return self._model.element("ISQMechanics::ModulusOfElasticityUnit")

    @property
    def ModulusOfElasticityValue(self):
        return self._model.element("ISQMechanics::ModulusOfElasticityValue")

    @property
    def ModulusOfImpedanceUnit(self):
        return self._model.element("ISQElectromagnetism::ModulusOfImpedanceUnit")

    @property
    def ModulusOfImpedanceValue(self):
        return self._model.element("ISQElectromagnetism::ModulusOfImpedanceValue")

    @property
    def ModulusOfRigidityUnit(self):
        return self._model.element("ISQMechanics::ModulusOfRigidityUnit")

    @property
    def ModulusOfRigidityValue(self):
        return self._model.element("ISQMechanics::ModulusOfRigidityValue")

    @property
    def MolalityUnit(self):
        return self._model.element("ISQChemistryMolecular::MolalityUnit")

    @property
    def MolalityValue(self):
        return self._model.element("ISQChemistryMolecular::MolalityValue")

    @property
    def MolarAbsorptionCoefficientUnit(self):
        return self._model.element("ISQLight::MolarAbsorptionCoefficientUnit")

    @property
    def MolarAbsorptionCoefficientValue(self):
        return self._model.element("ISQLight::MolarAbsorptionCoefficientValue")

    @property
    def MolarAttenuationCoefficientUnit(self):
        return self._model.element("ISQAtomicNuclear::MolarAttenuationCoefficientUnit")

    @property
    def MolarAttenuationCoefficientValue(self):
        return self._model.element("ISQAtomicNuclear::MolarAttenuationCoefficientValue")

    @property
    def MolarConductivityUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarConductivityUnit")

    @property
    def MolarConductivityValue(self):
        return self._model.element("ISQChemistryMolecular::MolarConductivityValue")

    @property
    def MolarEnthalpyUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarEnthalpyUnit")

    @property
    def MolarEnthalpyValue(self):
        return self._model.element("ISQChemistryMolecular::MolarEnthalpyValue")

    @property
    def MolarEntropyUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarEntropyUnit")

    @property
    def MolarEntropyValue(self):
        return self._model.element("ISQChemistryMolecular::MolarEntropyValue")

    @property
    def MolarGasConstantUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarGasConstantUnit")

    @property
    def MolarGasConstantValue(self):
        return self._model.element("ISQChemistryMolecular::MolarGasConstantValue")

    @property
    def MolarGibbsEnergyUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarGibbsEnergyUnit")

    @property
    def MolarGibbsEnergyValue(self):
        return self._model.element("ISQChemistryMolecular::MolarGibbsEnergyValue")

    @property
    def MolarHeatCapacityUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarHeatCapacityUnit")

    @property
    def MolarHeatCapacityValue(self):
        return self._model.element("ISQChemistryMolecular::MolarHeatCapacityValue")

    @property
    def MolarHelmholtzEnergyUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarHelmholtzEnergyUnit")

    @property
    def MolarHelmholtzEnergyValue(self):
        return self._model.element("ISQChemistryMolecular::MolarHelmholtzEnergyValue")

    @property
    def MolarInternalEnergyUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarInternalEnergyUnit")

    @property
    def MolarInternalEnergyValue(self):
        return self._model.element("ISQChemistryMolecular::MolarInternalEnergyValue")

    @property
    def MolarMassUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarMassUnit")

    @property
    def MolarMassValue(self):
        return self._model.element("ISQChemistryMolecular::MolarMassValue")

    @property
    def MolarOpticalRotatoryPowerUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")

    @property
    def MolarOpticalRotatoryPowerValue(self):
        return self._model.element("ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")

    @property
    def MolarVolumeUnit(self):
        return self._model.element("ISQChemistryMolecular::MolarVolumeUnit")

    @property
    def MolarVolumeValue(self):
        return self._model.element("ISQChemistryMolecular::MolarVolumeValue")

    @property
    def MolecularPartitionFunctionValue(self):
        return self._model.element("ISQChemistryMolecular::MolecularPartitionFunctionValue")

    @property
    def MomentOfForceUnit(self):
        return self._model.element("ISQMechanics::MomentOfForceUnit")

    @property
    def MomentOfForceValue(self):
        return self._model.element("ISQMechanics::MomentOfForceValue")

    @property
    def MomentOfInertiaUnit(self):
        return self._model.element("ISQMechanics::MomentOfInertiaUnit")

    @property
    def MomentOfInertiaValue(self):
        return self._model.element("ISQMechanics::MomentOfInertiaValue")

    @property
    def MomentumUnit(self):
        return self._model.element("ISQMechanics::MomentumUnit")

    @property
    def MomentumValue(self):
        return self._model.element("ISQMechanics::MomentumValue")

    @property
    def MortonNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::MortonNumberValue")

    @property
    def MultiplicationFactorUnit(self):
        return self._model.element("ISQAtomicNuclear::MultiplicationFactorUnit")

    @property
    def MultiplicationFactorValue(self):
        return self._model.element("ISQAtomicNuclear::MultiplicationFactorValue")

    @property
    def N(self):
        return self._model.element("ISQBase::'International System of Quantities'::N")

    @property
    def NapierianAbsorbanceValue(self):
        return self._model.element("ISQLight::NapierianAbsorbanceValue")

    @property
    def NazeNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::NazeNumberValue")

    @property
    def NonLeakageProbabilityUnit(self):
        return self._model.element("ISQAtomicNuclear::NonLeakageProbabilityUnit")

    @property
    def NonLeakageProbabilityValue(self):
        return self._model.element("ISQAtomicNuclear::NonLeakageProbabilityValue")

    @property
    def NormalStressUnit(self):
        return self._model.element("ISQMechanics::NormalStressUnit")

    @property
    def NormalStressValue(self):
        return self._model.element("ISQMechanics::NormalStressValue")

    @property
    def NuclearActivityUnit(self):
        return self._model.element("ISQAtomicNuclear::NuclearActivityUnit")

    @property
    def NuclearActivityValue(self):
        return self._model.element("ISQAtomicNuclear::NuclearActivityValue")

    @property
    def NuclearQuadrupoleMomentUnit(self):
        return self._model.element("ISQAtomicNuclear::NuclearQuadrupoleMomentUnit")

    @property
    def NuclearQuadrupoleMomentValue(self):
        return self._model.element("ISQAtomicNuclear::NuclearQuadrupoleMomentValue")

    @property
    def NusseltElectricNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::NusseltElectricNumberValue")

    @property
    def NusseltNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::NusseltNumberForMassTransferValue")

    @property
    def NusseltNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::NusseltNumberValue")

    @property
    def OhnesorgeNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::OhnesorgeNumberValue")

    @property
    def OsmoticFactorOfSolventValue(self):
        return self._model.element("ISQChemistryMolecular::OsmoticFactorOfSolventValue")

    @property
    def OsmoticPressureUnit(self):
        return self._model.element("ISQChemistryMolecular::OsmoticPressureUnit")

    @property
    def OsmoticPressureValue(self):
        return self._model.element("ISQChemistryMolecular::OsmoticPressureValue")

    @property
    def P_cletNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::'PécletNumberForMassTransferValue'")

    @property
    def P_cletNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::'PécletNumberValue'")

    @property
    def PackingFractionValue(self):
        return self._model.element("ISQAtomicNuclear::PackingFractionValue")

    @property
    def PartialPressureUnit(self):
        return self._model.element("ISQChemistryMolecular::PartialPressureUnit")

    @property
    def PartialPressureValue(self):
        return self._model.element("ISQChemistryMolecular::PartialPressureValue")

    @property
    def ParticleConcentrationUnit(self):
        return self._model.element("ISQChemistryMolecular::ParticleConcentrationUnit")

    @property
    def ParticleConcentrationValue(self):
        return self._model.element("ISQChemistryMolecular::ParticleConcentrationValue")

    @property
    def ParticleCurrentDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::ParticleCurrentDensityUnit")

    @property
    def ParticleCurrentDensityValue(self):
        return self._model.element("ISQAtomicNuclear::ParticleCurrentDensityValue")

    @property
    def ParticleEmissionRateUnit(self):
        return self._model.element("ISQAtomicNuclear::ParticleEmissionRateUnit")

    @property
    def ParticleEmissionRateValue(self):
        return self._model.element("ISQAtomicNuclear::ParticleEmissionRateValue")

    @property
    def ParticleFluenceRateUnit(self):
        return self._model.element("ISQAtomicNuclear::ParticleFluenceRateUnit")

    @property
    def ParticleFluenceRateValue(self):
        return self._model.element("ISQAtomicNuclear::ParticleFluenceRateValue")

    @property
    def ParticleFluenceUnit(self):
        return self._model.element("ISQAtomicNuclear::ParticleFluenceUnit")

    @property
    def ParticleFluenceValue(self):
        return self._model.element("ISQAtomicNuclear::ParticleFluenceValue")

    @property
    def ParticleNumberDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::ParticleNumberDensityUnit")

    @property
    def ParticleNumberDensityValue(self):
        return self._model.element("ISQAtomicNuclear::ParticleNumberDensityValue")

    @property
    def ParticleSourceDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::ParticleSourceDensityUnit")

    @property
    def ParticleSourceDensityValue(self):
        return self._model.element("ISQAtomicNuclear::ParticleSourceDensityValue")

    @property
    def PermeabilityUnit(self):
        return self._model.element("ISQElectromagnetism::PermeabilityUnit")

    @property
    def PermeabilityValue(self):
        return self._model.element("ISQElectromagnetism::PermeabilityValue")

    @property
    def PermeanceUnit(self):
        return self._model.element("ISQElectromagnetism::PermeanceUnit")

    @property
    def PermeanceValue(self):
        return self._model.element("ISQElectromagnetism::PermeanceValue")

    @property
    def PermittivityUnit(self):
        return self._model.element("ISQElectromagnetism::PermittivityUnit")

    @property
    def PermittivityValue(self):
        return self._model.element("ISQElectromagnetism::PermittivityValue")

    @property
    def PhaseCoefficientUnit(self):
        return self._model.element("ISQSpaceTime::PhaseCoefficientUnit")

    @property
    def PhaseCoefficientValue(self):
        return self._model.element("ISQSpaceTime::PhaseCoefficientValue")

    @property
    def PhaseDifferenceUnit(self):
        return self._model.element("ISQElectromagnetism::PhaseDifferenceUnit")

    @property
    def PhaseDifferenceValue(self):
        return self._model.element("ISQElectromagnetism::PhaseDifferenceValue")

    @property
    def PhaseSpeedOfElectromagneticWavesUnit(self):
        return self._model.element("ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit")

    @property
    def PhaseSpeedOfElectromagneticWavesValue(self):
        return self._model.element("ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue")

    @property
    def PhaseVelocityUnit(self):
        return self._model.element("ISQSpaceTime::PhaseVelocityUnit")

    @property
    def PhaseVelocityValue(self):
        return self._model.element("ISQSpaceTime::PhaseVelocityValue")

    @property
    def PhotonExitanceUnit(self):
        return self._model.element("ISQLight::PhotonExitanceUnit")

    @property
    def PhotonExitanceValue(self):
        return self._model.element("ISQLight::PhotonExitanceValue")

    @property
    def PhotonExposureUnit(self):
        return self._model.element("ISQLight::PhotonExposureUnit")

    @property
    def PhotonExposureValue(self):
        return self._model.element("ISQLight::PhotonExposureValue")

    @property
    def PhotonFluxUnit(self):
        return self._model.element("ISQLight::PhotonFluxUnit")

    @property
    def PhotonFluxValue(self):
        return self._model.element("ISQLight::PhotonFluxValue")

    @property
    def PhotonIntensityUnit(self):
        return self._model.element("ISQLight::PhotonIntensityUnit")

    @property
    def PhotonIntensityValue(self):
        return self._model.element("ISQLight::PhotonIntensityValue")

    @property
    def PhotonIrradianceUnit(self):
        return self._model.element("ISQLight::PhotonIrradianceUnit")

    @property
    def PhotonIrradianceValue(self):
        return self._model.element("ISQLight::PhotonIrradianceValue")

    @property
    def PhotonNumberValue(self):
        return self._model.element("ISQLight::PhotonNumberValue")

    @property
    def PhotonRadianceUnit(self):
        return self._model.element("ISQLight::PhotonRadianceUnit")

    @property
    def PhotonRadianceValue(self):
        return self._model.element("ISQLight::PhotonRadianceValue")

    @property
    def PlanckFunctionUnit(self):
        return self._model.element("ISQThermodynamics::PlanckFunctionUnit")

    @property
    def PlanckFunctionValue(self):
        return self._model.element("ISQThermodynamics::PlanckFunctionValue")

    @property
    def PlanetaryPosition3dVector(self):
        return self._model.element("ISQSpaceTime::PlanetaryPosition3dVector")

    @property
    def PlanetarySpatial3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")

    @property
    def PoiseuilleNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::PoiseuilleNumberValue")

    @property
    def PoissonNumberValue(self):
        return self._model.element("ISQMechanics::PoissonNumberValue")

    @property
    def PomerantsevNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::PomerantsevNumberValue")

    @property
    def Position3dVector(self):
        return self._model.element("ISQSpaceTime::Position3dVector")

    @property
    def PowerFactorValue(self):
        return self._model.element("ISQElectromagnetism::PowerFactorValue")

    @property
    def PowerNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::PowerNumberValue")

    @property
    def PowerUnit(self):
        return self._model.element("ISQMechanics::PowerUnit")

    @property
    def PowerValue(self):
        return self._model.element("ISQMechanics::PowerValue")

    @property
    def PoyntingVectorMagnitudeUnit(self):
        return self._model.element("ISQElectromagnetism::PoyntingVectorMagnitudeUnit")

    @property
    def PoyntingVectorMagnitudeValue(self):
        return self._model.element("ISQElectromagnetism::PoyntingVectorMagnitudeValue")

    @property
    def PrandtlMagneticNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::PrandtlMagneticNumberValue")

    @property
    def PrandtlNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::PrandtlNumberValue")

    @property
    def PressureCoefficientUnit(self):
        return self._model.element("ISQThermodynamics::PressureCoefficientUnit")

    @property
    def PressureCoefficientValue(self):
        return self._model.element("ISQThermodynamics::PressureCoefficientValue")

    @property
    def PressureUnit(self):
        return self._model.element("ISQMechanics::PressureUnit")

    @property
    def PressureValue(self):
        return self._model.element("ISQMechanics::PressureValue")

    @property
    def PropagationCoefficientUnit(self):
        return self._model.element("ISQSpaceTime::PropagationCoefficientUnit")

    @property
    def PropagationCoefficientValue(self):
        return self._model.element("ISQSpaceTime::PropagationCoefficientValue")

    @property
    def QualityFactorForIonizingRadiationUnit(self):
        return self._model.element("ISQAtomicNuclear::QualityFactorForIonizingRadiationUnit")

    @property
    def QualityFactorForIonizingRadiationValue(self):
        return self._model.element("ISQAtomicNuclear::QualityFactorForIonizingRadiationValue")

    @property
    def QualityFactorValue(self):
        return self._model.element("ISQElectromagnetism::QualityFactorValue")

    @property
    def QuantumNumberValue(self):
        return self._model.element("ISQAtomicNuclear::QuantumNumberValue")

    @property
    def RadianceFactorValue(self):
        return self._model.element("ISQLight::RadianceFactorValue")

    @property
    def RadianceUnit(self):
        return self._model.element("ISQLight::RadianceUnit")

    @property
    def RadianceValue(self):
        return self._model.element("ISQLight::RadianceValue")

    @property
    def RadiantEnergyDensityUnit(self):
        return self._model.element("ISQLight::RadiantEnergyDensityUnit")

    @property
    def RadiantEnergyDensityValue(self):
        return self._model.element("ISQLight::RadiantEnergyDensityValue")

    @property
    def RadiantExitanceUnit(self):
        return self._model.element("ISQLight::RadiantExitanceUnit")

    @property
    def RadiantExitanceValue(self):
        return self._model.element("ISQLight::RadiantExitanceValue")

    @property
    def RadiantExposureUnit(self):
        return self._model.element("ISQLight::RadiantExposureUnit")

    @property
    def RadiantExposureValue(self):
        return self._model.element("ISQLight::RadiantExposureValue")

    @property
    def RadiantFluxUnit(self):
        return self._model.element("ISQLight::RadiantFluxUnit")

    @property
    def RadiantFluxValue(self):
        return self._model.element("ISQLight::RadiantFluxValue")

    @property
    def RadiantIntensityUnit(self):
        return self._model.element("ISQLight::RadiantIntensityUnit")

    @property
    def RadiantIntensityValue(self):
        return self._model.element("ISQLight::RadiantIntensityValue")

    @property
    def RatioOfSpecificHeatCapacitiesValue(self):
        return self._model.element("ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue")

    @property
    def RayleighNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::RayleighNumberValue")

    @property
    def ReactanceUnit(self):
        return self._model.element("ISQElectromagnetism::ReactanceUnit")

    @property
    def ReactanceValue(self):
        return self._model.element("ISQElectromagnetism::ReactanceValue")

    @property
    def RecombinationCoefficientUnit(self):
        return self._model.element("ISQAtomicNuclear::RecombinationCoefficientUnit")

    @property
    def RecombinationCoefficientValue(self):
        return self._model.element("ISQAtomicNuclear::RecombinationCoefficientValue")

    @property
    def RedundancyUnit(self):
        return self._model.element("ISQInformation::RedundancyUnit")

    @property
    def RedundancyValue(self):
        return self._model.element("ISQInformation::RedundancyValue")

    @property
    def ReechNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ReechNumberValue")

    @property
    def ReflectanceFactorValue(self):
        return self._model.element("ISQLight::ReflectanceFactorValue")

    @property
    def ReflectanceValue(self):
        return self._model.element("ISQLight::ReflectanceValue")

    @property
    def RefractiveIndexValue(self):
        return self._model.element("ISQLight::RefractiveIndexValue")

    @property
    def RelativeAtomicMassValue(self):
        return self._model.element("ISQChemistryMolecular::RelativeAtomicMassValue")

    @property
    def RelativeEntropyValue(self):
        return self._model.element("ISQInformation::RelativeEntropyValue")

    @property
    def RelativeHumidityValue(self):
        return self._model.element("ISQThermodynamics::RelativeHumidityValue")

    @property
    def RelativeLinearStrainValue(self):
        return self._model.element("ISQMechanics::RelativeLinearStrainValue")

    @property
    def RelativeMassConcentrationOfVapourValue(self):
        return self._model.element("ISQThermodynamics::RelativeMassConcentrationOfVapourValue")

    @property
    def RelativeMassDefectValue(self):
        return self._model.element("ISQAtomicNuclear::RelativeMassDefectValue")

    @property
    def RelativeMassDensityValue(self):
        return self._model.element("ISQMechanics::RelativeMassDensityValue")

    @property
    def RelativeMassExcessValue(self):
        return self._model.element("ISQAtomicNuclear::RelativeMassExcessValue")

    @property
    def RelativeMassRatioOfVapourValue(self):
        return self._model.element("ISQThermodynamics::RelativeMassRatioOfVapourValue")

    @property
    def RelativePermeabilityValue(self):
        return self._model.element("ISQElectromagnetism::RelativePermeabilityValue")

    @property
    def RelativePermittivityValue(self):
        return self._model.element("ISQElectromagnetism::RelativePermittivityValue")

    @property
    def RelativePressureCoefficientUnit(self):
        return self._model.element("ISQThermodynamics::RelativePressureCoefficientUnit")

    @property
    def RelativePressureCoefficientValue(self):
        return self._model.element("ISQThermodynamics::RelativePressureCoefficientValue")

    @property
    def RelativeRedundancyValue(self):
        return self._model.element("ISQInformation::RelativeRedundancyValue")

    @property
    def RelativeVolumeStrainValue(self):
        return self._model.element("ISQMechanics::RelativeVolumeStrainValue")

    @property
    def ReluctanceUnit(self):
        return self._model.element("ISQElectromagnetism::ReluctanceUnit")

    @property
    def ReluctanceValue(self):
        return self._model.element("ISQElectromagnetism::ReluctanceValue")

    @property
    def RepetencyUnit(self):
        return self._model.element("ISQSpaceTime::RepetencyUnit")

    @property
    def RepetencyValue(self):
        return self._model.element("ISQSpaceTime::RepetencyValue")

    @property
    def ResistanceToAlternatingCurrentUnit(self):
        return self._model.element("ISQElectromagnetism::ResistanceToAlternatingCurrentUnit")

    @property
    def ResistanceToAlternatingCurrentValue(self):
        return self._model.element("ISQElectromagnetism::ResistanceToAlternatingCurrentValue")

    @property
    def ResistanceUnit(self):
        return self._model.element("ISQElectromagnetism::ResistanceUnit")

    @property
    def ResistanceValue(self):
        return self._model.element("ISQElectromagnetism::ResistanceValue")

    @property
    def ResistivityUnit(self):
        return self._model.element("ISQElectromagnetism::ResistivityUnit")

    @property
    def ResistivityValue(self):
        return self._model.element("ISQElectromagnetism::ResistivityValue")

    @property
    def ResonanceEscapeProbabilityValue(self):
        return self._model.element("ISQAtomicNuclear::ResonanceEscapeProbabilityValue")

    @property
    def ReynoldsElectricNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ReynoldsElectricNumberValue")

    @property
    def ReynoldsMagneticNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ReynoldsMagneticNumberValue")

    @property
    def ReynoldsNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::ReynoldsNumberValue")

    @property
    def RichardsonConstantUnit(self):
        return self._model.element("ISQCondensedMatter::RichardsonConstantUnit")

    @property
    def RichardsonConstantValue(self):
        return self._model.element("ISQCondensedMatter::RichardsonConstantValue")

    @property
    def RichardsonNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::RichardsonNumberValue")

    @property
    def RobertsNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::RobertsNumberValue")

    @property
    def RollingResistanceFactorValue(self):
        return self._model.element("ISQMechanics::RollingResistanceFactorValue")

    @property
    def RossbyNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::RossbyNumberValue")

    @property
    def RydbergConstantUnit(self):
        return self._model.element("ISQAtomicNuclear::RydbergConstantUnit")

    @property
    def RydbergConstantValue(self):
        return self._model.element("ISQAtomicNuclear::RydbergConstantValue")

    @property
    def SchmidtNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::SchmidtNumberValue")

    @property
    def SecondAxialMomentOfAreaUnit(self):
        return self._model.element("ISQMechanics::SecondAxialMomentOfAreaUnit")

    @property
    def SecondAxialMomentOfAreaValue(self):
        return self._model.element("ISQMechanics::SecondAxialMomentOfAreaValue")

    @property
    def SecondPolarMomentOfAreaUnit(self):
        return self._model.element("ISQMechanics::SecondPolarMomentOfAreaUnit")

    @property
    def SecondPolarMomentOfAreaValue(self):
        return self._model.element("ISQMechanics::SecondPolarMomentOfAreaValue")

    @property
    def SectionModulusUnit(self):
        return self._model.element("ISQMechanics::SectionModulusUnit")

    @property
    def SectionModulusValue(self):
        return self._model.element("ISQMechanics::SectionModulusValue")

    @property
    def SeebeckCoefficientForSubstancesAAndBUnit(self):
        return self._model.element("ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit")

    @property
    def SeebeckCoefficientForSubstancesAAndBValue(self):
        return self._model.element("ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue")

    @property
    def ShearStrainValue(self):
        return self._model.element("ISQMechanics::ShearStrainValue")

    @property
    def ShearStressUnit(self):
        return self._model.element("ISQMechanics::ShearStressUnit")

    @property
    def ShearStressValue(self):
        return self._model.element("ISQMechanics::ShearStressValue")

    @property
    def ShortRangeOrderParameterValue(self):
        return self._model.element("ISQCondensedMatter::ShortRangeOrderParameterValue")

    @property
    def SlowingDownDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::SlowingDownDensityUnit")

    @property
    def SlowingDownDensityValue(self):
        return self._model.element("ISQAtomicNuclear::SlowingDownDensityValue")

    @property
    def SolidAngularMeasureUnit(self):
        return self._model.element("ISQSpaceTime::SolidAngularMeasureUnit")

    @property
    def SolidAngularMeasureValue(self):
        return self._model.element("ISQSpaceTime::SolidAngularMeasureValue")

    @property
    def SommerfeldNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::SommerfeldNumberValue")

    @property
    def SoundEnergyDensityUnit(self):
        return self._model.element("ISQAcoustics::SoundEnergyDensityUnit")

    @property
    def SoundEnergyDensityValue(self):
        return self._model.element("ISQAcoustics::SoundEnergyDensityValue")

    @property
    def SoundExposureLevelUnit(self):
        return self._model.element("ISQAcoustics::SoundExposureLevelUnit")

    @property
    def SoundExposureLevelValue(self):
        return self._model.element("ISQAcoustics::SoundExposureLevelValue")

    @property
    def SoundExposureUnit(self):
        return self._model.element("ISQAcoustics::SoundExposureUnit")

    @property
    def SoundExposureValue(self):
        return self._model.element("ISQAcoustics::SoundExposureValue")

    @property
    def SoundIntensityUnit(self):
        return self._model.element("ISQAcoustics::SoundIntensityUnit")

    @property
    def SoundIntensityValue(self):
        return self._model.element("ISQAcoustics::SoundIntensityValue")

    @property
    def SoundPowerLevelUnit(self):
        return self._model.element("ISQAcoustics::SoundPowerLevelUnit")

    @property
    def SoundPowerLevelValue(self):
        return self._model.element("ISQAcoustics::SoundPowerLevelValue")

    @property
    def SoundPressureLevelUnit(self):
        return self._model.element("ISQAcoustics::SoundPressureLevelUnit")

    @property
    def SoundPressureLevelValue(self):
        return self._model.element("ISQAcoustics::SoundPressureLevelValue")

    @property
    def SourceVoltageUnit(self):
        return self._model.element("ISQElectromagnetism::SourceVoltageUnit")

    @property
    def SourceVoltageValue(self):
        return self._model.element("ISQElectromagnetism::SourceVoltageValue")

    @property
    def Spatial3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::Spatial3dCoordinateFrame")

    @property
    def SpecificActivityUnit(self):
        return self._model.element("ISQAtomicNuclear::SpecificActivityUnit")

    @property
    def SpecificActivityValue(self):
        return self._model.element("ISQAtomicNuclear::SpecificActivityValue")

    @property
    def SpecificEnergyUnit(self):
        return self._model.element("ISQThermodynamics::SpecificEnergyUnit")

    @property
    def SpecificEnergyValue(self):
        return self._model.element("ISQThermodynamics::SpecificEnergyValue")

    @property
    def SpecificEnthalpyUnit(self):
        return self._model.element("ISQThermodynamics::SpecificEnthalpyUnit")

    @property
    def SpecificEnthalpyValue(self):
        return self._model.element("ISQThermodynamics::SpecificEnthalpyValue")

    @property
    def SpecificEntropyUnit(self):
        return self._model.element("ISQThermodynamics::SpecificEntropyUnit")

    @property
    def SpecificEntropyValue(self):
        return self._model.element("ISQThermodynamics::SpecificEntropyValue")

    @property
    def SpecificGasConstantUnit(self):
        return self._model.element("ISQThermodynamics::SpecificGasConstantUnit")

    @property
    def SpecificGasConstantValue(self):
        return self._model.element("ISQThermodynamics::SpecificGasConstantValue")

    @property
    def SpecificHeatCapacityAtConstantPressureUnit(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit")

    @property
    def SpecificHeatCapacityAtConstantPressureValue(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue")

    @property
    def SpecificHeatCapacityAtConstantVolumeUnit(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit")

    @property
    def SpecificHeatCapacityAtConstantVolumeValue(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue")

    @property
    def SpecificHeatCapacityAtSaturatedVapourPressureUnit(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit")

    @property
    def SpecificHeatCapacityAtSaturatedVapourPressureValue(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue")

    @property
    def SpecificHeatCapacityUnit(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityUnit")

    @property
    def SpecificHeatCapacityValue(self):
        return self._model.element("ISQThermodynamics::SpecificHeatCapacityValue")

    @property
    def SpecificOpticalRotatoryPowerUnit(self):
        return self._model.element("ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")

    @property
    def SpecificOpticalRotatoryPowerValue(self):
        return self._model.element("ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")

    @property
    def SpecificVolumeUnit(self):
        return self._model.element("ISQMechanics::SpecificVolumeUnit")

    @property
    def SpecificVolumeValue(self):
        return self._model.element("ISQMechanics::SpecificVolumeValue")

    @property
    def SpectralIrradianceUnit(self):
        return self._model.element("ISQLight::SpectralIrradianceUnit")

    @property
    def SpectralIrradianceValue(self):
        return self._model.element("ISQLight::SpectralIrradianceValue")

    @property
    def SpectralLuminousEfficacyUnit(self):
        return self._model.element("ISQLight::SpectralLuminousEfficacyUnit")

    @property
    def SpectralLuminousEfficacyValue(self):
        return self._model.element("ISQLight::SpectralLuminousEfficacyValue")

    @property
    def SpectralLuminousEfficiencyValue(self):
        return self._model.element("ISQLight::SpectralLuminousEfficiencyValue")

    @property
    def SpectralRadianceUnit(self):
        return self._model.element("ISQLight::SpectralRadianceUnit")

    @property
    def SpectralRadianceValue(self):
        return self._model.element("ISQLight::SpectralRadianceValue")

    @property
    def SpectralRadiantEnergyDensityInTermsOfWavelengthUnit(self):
        return self._model.element("ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")

    @property
    def SpectralRadiantEnergyDensityInTermsOfWavelengthValue(self):
        return self._model.element("ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue")

    @property
    def SpectralRadiantEnergyDensityInTermsOfWavenumberUnit(self):
        return self._model.element("ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit")

    @property
    def SpectralRadiantEnergyDensityInTermsOfWavenumberValue(self):
        return self._model.element("ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue")

    @property
    def SpectralRadiantEnergyUnit(self):
        return self._model.element("ISQLight::SpectralRadiantEnergyUnit")

    @property
    def SpectralRadiantEnergyValue(self):
        return self._model.element("ISQLight::SpectralRadiantEnergyValue")

    @property
    def SpectralRadiantExitanceUnit(self):
        return self._model.element("ISQLight::SpectralRadiantExitanceUnit")

    @property
    def SpectralRadiantExitanceValue(self):
        return self._model.element("ISQLight::SpectralRadiantExitanceValue")

    @property
    def SpectralRadiantExposureUnit(self):
        return self._model.element("ISQLight::SpectralRadiantExposureUnit")

    @property
    def SpectralRadiantExposureValue(self):
        return self._model.element("ISQLight::SpectralRadiantExposureValue")

    @property
    def SpectralRadiantFluxUnit(self):
        return self._model.element("ISQLight::SpectralRadiantFluxUnit")

    @property
    def SpectralRadiantFluxValue(self):
        return self._model.element("ISQLight::SpectralRadiantFluxValue")

    @property
    def SpectralRadiantIntensityUnit(self):
        return self._model.element("ISQLight::SpectralRadiantIntensityUnit")

    @property
    def SpectralRadiantIntensityValue(self):
        return self._model.element("ISQLight::SpectralRadiantIntensityValue")

    @property
    def SpeedOfLightInAMediumUnit(self):
        return self._model.element("ISQLight::SpeedOfLightInAMediumUnit")

    @property
    def SpeedOfLightInAMediumValue(self):
        return self._model.element("ISQLight::SpeedOfLightInAMediumValue")

    @property
    def SpeedOfLightUnit(self):
        return self._model.element("ISQElectromagnetism::SpeedOfLightUnit")

    @property
    def SpeedOfLightValue(self):
        return self._model.element("ISQElectromagnetism::SpeedOfLightValue")

    @property
    def SpeedUnit(self):
        return self._model.element("ISQSpaceTime::SpeedUnit")

    @property
    def SpeedValue(self):
        return self._model.element("ISQSpaceTime::SpeedValue")

    @property
    def SphericalDisplacement3dVector(self):
        return self._model.element("ISQSpaceTime::SphericalDisplacement3dVector")

    @property
    def SphericalPosition3dVector(self):
        return self._model.element("ISQSpaceTime::SphericalPosition3dVector")

    @property
    def SphericalSpatial3dCoordinateFrame(self):
        return self._model.element("ISQSpaceTime::SphericalSpatial3dCoordinateFrame")

    @property
    def SpinUnit(self):
        return self._model.element("ISQAtomicNuclear::SpinUnit")

    @property
    def SpinValue(self):
        return self._model.element("ISQAtomicNuclear::SpinValue")

    @property
    def StandardAbsoluteActivityInMixtureValue(self):
        return self._model.element("ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue")

    @property
    def StandardAbsoluteActivityInSolutionValue(self):
        return self._model.element("ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue")

    @property
    def StandardAbsoluteActivityOfSolventValue(self):
        return self._model.element("ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue")

    @property
    def StandardChemicalPotentialUnit(self):
        return self._model.element("ISQChemistryMolecular::StandardChemicalPotentialUnit")

    @property
    def StandardChemicalPotentialValue(self):
        return self._model.element("ISQChemistryMolecular::StandardChemicalPotentialValue")

    @property
    def StandardEquilibriumConstantValue(self):
        return self._model.element("ISQChemistryMolecular::StandardEquilibriumConstantValue")

    @property
    def StantonNumberForMassTransferValue(self):
        return self._model.element("ISQCharacteristicNumbers::StantonNumberForMassTransferValue")

    @property
    def StantonNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StantonNumberValue")

    @property
    def StarkNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StarkNumberValue")

    @property
    def StaticFrictionCoefficientValue(self):
        return self._model.element("ISQMechanics::StaticFrictionCoefficientValue")

    @property
    def StefanNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StefanNumberValue")

    @property
    def StoichiometricNumberOfSubstanceValue(self):
        return self._model.element("ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue")

    @property
    def StokesNumberForDragValue(self):
        return self._model.element("ISQCharacteristicNumbers::StokesNumberForDragValue")

    @property
    def StokesNumberForGravityValue(self):
        return self._model.element("ISQCharacteristicNumbers::StokesNumberForGravityValue")

    @property
    def StokesNumberForRotameterValue(self):
        return self._model.element("ISQCharacteristicNumbers::StokesNumberForRotameterValue")

    @property
    def StokesNumberForVibratingParticlesValue(self):
        return self._model.element("ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue")

    @property
    def StokesNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StokesNumberValue")

    @property
    def StorageCapacityUnit(self):
        return self._model.element("ISQInformation::StorageCapacityUnit")

    @property
    def StorageCapacityValue(self):
        return self._model.element("ISQInformation::StorageCapacityValue")

    @property
    def StrainUnit(self):
        return self._model.element("ISQMechanics::StrainUnit")

    @property
    def StrainValue(self):
        return self._model.element("ISQMechanics::StrainValue")

    @property
    def StressUnit(self):
        return self._model.element("ISQMechanics::StressUnit")

    @property
    def StressValue(self):
        return self._model.element("ISQMechanics::StressValue")

    @property
    def StrouhalNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StrouhalNumberValue")

    @property
    def StructureFactorValue(self):
        return self._model.element("ISQCondensedMatter::StructureFactorValue")

    @property
    def StuartElectricalNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StuartElectricalNumberValue")

    @property
    def StuartNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::StuartNumberValue")

    @property
    def SurfaceActivityDensityUnit(self):
        return self._model.element("ISQAtomicNuclear::SurfaceActivityDensityUnit")

    @property
    def SurfaceActivityDensityValue(self):
        return self._model.element("ISQAtomicNuclear::SurfaceActivityDensityValue")

    @property
    def SurfaceCoefficientOfHeatTransferUnit(self):
        return self._model.element("ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit")

    @property
    def SurfaceCoefficientOfHeatTransferValue(self):
        return self._model.element("ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue")

    @property
    def SurfaceDensityOfElectricChargeUnit(self):
        return self._model.element("ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit")

    @property
    def SurfaceDensityOfElectricChargeValue(self):
        return self._model.element("ISQElectromagnetism::SurfaceDensityOfElectricChargeValue")

    @property
    def SurfaceMassDensityUnit(self):
        return self._model.element("ISQMechanics::SurfaceMassDensityUnit")

    @property
    def SurfaceMassDensityValue(self):
        return self._model.element("ISQMechanics::SurfaceMassDensityValue")

    @property
    def SurfaceTensionUnit(self):
        return self._model.element("ISQMechanics::SurfaceTensionUnit")

    @property
    def SurfaceTensionValue(self):
        return self._model.element("ISQMechanics::SurfaceTensionValue")

    @property
    def SusceptanceUnit(self):
        return self._model.element("ISQElectromagnetism::SusceptanceUnit")

    @property
    def SusceptanceValue(self):
        return self._model.element("ISQElectromagnetism::SusceptanceValue")

    @property
    def T(self):
        return self._model.element("ISQBase::'International System of Quantities'::T")

    @property
    def TaylorNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::TaylorNumberValue")

    @property
    def TemperatureDifferenceUnit(self):
        return self._model.element("ISQ::TemperatureDifferenceUnit")

    @property
    def TemperatureDifferenceValue(self):
        return self._model.element("ISQ::TemperatureDifferenceValue")

    @property
    def ThermalConductanceUnit(self):
        return self._model.element("ISQThermodynamics::ThermalConductanceUnit")

    @property
    def ThermalConductanceValue(self):
        return self._model.element("ISQThermodynamics::ThermalConductanceValue")

    @property
    def ThermalConductivityUnit(self):
        return self._model.element("ISQThermodynamics::ThermalConductivityUnit")

    @property
    def ThermalConductivityValue(self):
        return self._model.element("ISQThermodynamics::ThermalConductivityValue")

    @property
    def ThermalDiffusionCoefficientUnit(self):
        return self._model.element("ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")

    @property
    def ThermalDiffusionCoefficientValue(self):
        return self._model.element("ISQChemistryMolecular::ThermalDiffusionCoefficientValue")

    @property
    def ThermalDiffusionFactorValue(self):
        return self._model.element("ISQChemistryMolecular::ThermalDiffusionFactorValue")

    @property
    def ThermalDiffusionRatioValue(self):
        return self._model.element("ISQChemistryMolecular::ThermalDiffusionRatioValue")

    @property
    def ThermalDiffusivityUnit(self):
        return self._model.element("ISQThermodynamics::ThermalDiffusivityUnit")

    @property
    def ThermalDiffusivityValue(self):
        return self._model.element("ISQThermodynamics::ThermalDiffusivityValue")

    @property
    def ThermalEfficiencyValue(self):
        return self._model.element("ISQThermodynamics::ThermalEfficiencyValue")

    @property
    def ThermalInsulanceUnit(self):
        return self._model.element("ISQThermodynamics::ThermalInsulanceUnit")

    @property
    def ThermalInsulanceValue(self):
        return self._model.element("ISQThermodynamics::ThermalInsulanceValue")

    @property
    def ThermalResistanceUnit(self):
        return self._model.element("ISQThermodynamics::ThermalResistanceUnit")

    @property
    def ThermalResistanceValue(self):
        return self._model.element("ISQThermodynamics::ThermalResistanceValue")

    @property
    def ThermalUtilizationFactorUnit(self):
        return self._model.element("ISQAtomicNuclear::ThermalUtilizationFactorUnit")

    @property
    def ThermalUtilizationFactorValue(self):
        return self._model.element("ISQAtomicNuclear::ThermalUtilizationFactorValue")

    @property
    def ThermodynamicGr_neisenParameterValue(self):
        return self._model.element("ISQCondensedMatter::'ThermodynamicGrüneisenParameterValue'")

    @property
    def ThermodynamicTemperatureUnit(self):
        return self._model.element("ISQBase::ThermodynamicTemperatureUnit")

    @property
    def ThermodynamicTemperatureValue(self):
        return self._model.element("ISQBase::ThermodynamicTemperatureValue")

    @property
    def ThomsonCoefficientUnit(self):
        return self._model.element("ISQCondensedMatter::ThomsonCoefficientUnit")

    @property
    def ThomsonCoefficientValue(self):
        return self._model.element("ISQCondensedMatter::ThomsonCoefficientValue")

    @property
    def ThrustCoefficientValue(self):
        return self._model.element("ISQCharacteristicNumbers::ThrustCoefficientValue")

    @property
    def TorqueUnit(self):
        return self._model.element("ISQMechanics::TorqueUnit")

    @property
    def TorqueValue(self):
        return self._model.element("ISQMechanics::TorqueValue")

    @property
    def TotalAngularMomentumUnit(self):
        return self._model.element("ISQAtomicNuclear::TotalAngularMomentumUnit")

    @property
    def TotalAngularMomentumValue(self):
        return self._model.element("ISQAtomicNuclear::TotalAngularMomentumValue")

    @property
    def TotalCurrentDensityUnit(self):
        return self._model.element("ISQElectromagnetism::TotalCurrentDensityUnit")

    @property
    def TotalCurrentDensityValue(self):
        return self._model.element("ISQElectromagnetism::TotalCurrentDensityValue")

    @property
    def TotalIonizationValue(self):
        return self._model.element("ISQAtomicNuclear::TotalIonizationValue")

    @property
    def TotalLinearStoppingPowerUnit(self):
        return self._model.element("ISQAtomicNuclear::TotalLinearStoppingPowerUnit")

    @property
    def TotalLinearStoppingPowerValue(self):
        return self._model.element("ISQAtomicNuclear::TotalLinearStoppingPowerValue")

    @property
    def TotalMassStoppingPowerUnit(self):
        return self._model.element("ISQAtomicNuclear::TotalMassStoppingPowerUnit")

    @property
    def TotalMassStoppingPowerValue(self):
        return self._model.element("ISQAtomicNuclear::TotalMassStoppingPowerValue")

    @property
    def TrafficCarriedIntensityUnit(self):
        return self._model.element("ISQInformation::TrafficCarriedIntensityUnit")

    @property
    def TrafficCarriedIntensityValue(self):
        return self._model.element("ISQInformation::TrafficCarriedIntensityValue")

    @property
    def TrafficIntensityUnit(self):
        return self._model.element("ISQInformation::TrafficIntensityUnit")

    @property
    def TrafficIntensityValue(self):
        return self._model.element("ISQInformation::TrafficIntensityValue")

    @property
    def TrafficOfferedIntensityUnit(self):
        return self._model.element("ISQInformation::TrafficOfferedIntensityUnit")

    @property
    def TrafficOfferedIntensityValue(self):
        return self._model.element("ISQInformation::TrafficOfferedIntensityValue")

    @property
    def TransferRateUnit(self):
        return self._model.element("ISQInformation::TransferRateUnit")

    @property
    def TransferRateValue(self):
        return self._model.element("ISQInformation::TransferRateValue")

    @property
    def TransinformationContentUnit(self):
        return self._model.element("ISQInformation::TransinformationContentUnit")

    @property
    def TransinformationContentValue(self):
        return self._model.element("ISQInformation::TransinformationContentValue")

    @property
    def TransmittanceOpticalDensityValue(self):
        return self._model.element("ISQLight::TransmittanceOpticalDensityValue")

    @property
    def TransmittanceValue(self):
        return self._model.element("ISQLight::TransmittanceValue")

    @property
    def TransportNumberOfTheIonBValue(self):
        return self._model.element("ISQChemistryMolecular::TransportNumberOfTheIonBValue")

    @property
    def TristimulusValuesForTheCie1931StandardColorimetricObserverUnit(self):
        return self._model.element("ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit")

    @property
    def TristimulusValuesForTheCie1931StandardColorimetricObserverValue(self):
        return self._model.element("ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue")

    @property
    def TristimulusValuesForTheCie1964StandardColorimetricObserverUnit(self):
        return self._model.element("ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit")

    @property
    def TristimulusValuesForTheCie1964StandardColorimetricObserverValue(self):
        return self._model.element("ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue")

    @property
    def VolumeFlowRateUnit(self):
        return self._model.element("ISQMechanics::VolumeFlowRateUnit")

    @property
    def VolumeFlowRateValue(self):
        return self._model.element("ISQMechanics::VolumeFlowRateValue")

    @property
    def VolumeFractionUnit(self):
        return self._model.element("ISQChemistryMolecular::VolumeFractionUnit")

    @property
    def VolumeFractionValue(self):
        return self._model.element("ISQChemistryMolecular::VolumeFractionValue")

    @property
    def VolumeUnit(self):
        return self._model.element("ISQSpaceTime::VolumeUnit")

    @property
    def VolumeValue(self):
        return self._model.element("ISQSpaceTime::VolumeValue")

    @property
    def VolumicCrossSectionUnit(self):
        return self._model.element("ISQAtomicNuclear::VolumicCrossSectionUnit")

    @property
    def VolumicCrossSectionValue(self):
        return self._model.element("ISQAtomicNuclear::VolumicCrossSectionValue")

    @property
    def VolumicTotalCrossSectionUnit(self):
        return self._model.element("ISQAtomicNuclear::VolumicTotalCrossSectionUnit")

    @property
    def VolumicTotalCrossSectionValue(self):
        return self._model.element("ISQAtomicNuclear::VolumicTotalCrossSectionValue")

    @property
    def WaitingProbabilityValue(self):
        return self._model.element("ISQInformation::WaitingProbabilityValue")

    @property
    def WeberNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::WeberNumberValue")

    @property
    def WeissenbergNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::WeissenbergNumberValue")

    @property
    def WomersleyNumberValue(self):
        return self._model.element("ISQCharacteristicNumbers::WomersleyNumberValue")

    @property
    def absoluteActivity(self):
        return self._model.element("ISQChemistryMolecular::absoluteActivity")

    @property
    def absorbedDose(self):
        return self._model.element("ISQAtomicNuclear::absorbedDose")

    @property
    def absorbedDoseRate(self):
        return self._model.element("ISQAtomicNuclear::absorbedDoseRate")

    @property
    def absorptance(self):
        return self._model.element("ISQLight::absorptance")

    @property
    def absorptionNumber(self):
        return self._model.element("ISQCharacteristicNumbers::absorptionNumber")

    @property
    def acceleration(self):
        return self._model.element("ISQSpaceTime::acceleration")

    @property
    def acceptorDensity(self):
        return self._model.element("ISQCondensedMatter::acceptorDensity")

    @property
    def acousticImpedance(self):
        return self._model.element("ISQAcoustics::acousticImpedance")

    @property
    def actionQuantity(self):
        return self._model.element("ISQMechanics::actionQuantity")

    @property
    def activeEnergy(self):
        return self._model.element("ISQElectromagnetism::activeEnergy")

    @property
    def activePower(self):
        return self._model.element("ISQElectromagnetism::activePower")

    @property
    def activityCoefficient(self):
        return self._model.element("ISQChemistryMolecular::activityCoefficient")

    @property
    def activityDensity(self):
        return self._model.element("ISQAtomicNuclear::activityDensity")

    @property
    def activityFactor(self):
        return self._model.element("ISQChemistryMolecular::activityFactor")

    @property
    def activityOfSolute(self):
        return self._model.element("ISQChemistryMolecular::activityOfSolute")

    @property
    def activityOfSolvent(self):
        return self._model.element("ISQChemistryMolecular::activityOfSolvent")

    @property
    def admittance(self):
        return self._model.element("ISQElectromagnetism::admittance")

    @property
    def affinityOfAChemicalReaction(self):
        return self._model.element("ISQChemistryMolecular::affinityOfAChemicalReaction")

    @property
    def alfv_nNumber(self):
        return self._model.element("ISQCharacteristicNumbers::'alfvénNumber'")

    @property
    def alphaDisintegrationEnergy(self):
        return self._model.element("ISQAtomicNuclear::alphaDisintegrationEnergy")

    @property
    def altitude(self):
        return self._model.element("ISQSpaceTime::PlanetaryPosition3dVector::altitude")

    @property
    def altitudeUnit(self):
        return self._model.element("ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit")

    @property
    def amountOfSubstance(self):
        return self._model.element("ISQBase::amountOfSubstance")

    @property
    def amountOfSubstanceConcentration(self):
        return self._model.element("ISQChemistryMolecular::amountOfSubstanceConcentration")

    @property
    def amountOfSubstanceFractionMoleFraction(self):
        return self._model.element("ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction")

    @property
    def amountOfSubstancePF(self):
        return self._model.element("ISQLight::MolarAbsorptionCoefficientUnit::amountOfSubstancePF")

    @property
    def amp_reNumber(self):
        return self._model.element("ISQCharacteristicNumbers::'ampèreNumber'")

    @property
    def angleOfOpticalRotation(self):
        return self._model.element("ISQChemistryMolecular::angleOfOpticalRotation")

    @property
    def angularAcceleration(self):
        return self._model.element("ISQSpaceTime::angularAcceleration")

    @property
    def angularFrequency(self):
        return self._model.element("ISQSpaceTime::angularFrequency")

    @property
    def angularImpulse(self):
        return self._model.element("ISQMechanics::angularImpulse")

    @property
    def angularMeasure(self):
        return self._model.element("ISQSpaceTime::angularMeasure")

    @property
    def angularMomentum(self):
        return self._model.element("ISQMechanics::angularMomentum")

    @property
    def angularReciprocalLatticeVectorMagnitude(self):
        return self._model.element("ISQCondensedMatter::angularReciprocalLatticeVectorMagnitude")

    @property
    def angularRepetency(self):
        return self._model.element("ISQSpaceTime::angularRepetency")

    @property
    def angularVelocity(self):
        return self._model.element("ISQSpaceTime::angularVelocity")

    @property
    def angularWavenumber(self):
        return self._model.element("ISQCondensedMatter::angularWavenumber")

    @property
    def apparentPower(self):
        return self._model.element("ISQElectromagnetism::apparentPower")

    @property
    def archimedesNumber(self):
        return self._model.element("ISQCharacteristicNumbers::archimedesNumber")

    @property
    def area(self):
        return self._model.element("ISQSpaceTime::area")

    @property
    def arrheniusNumber(self):
        return self._model.element("ISQCharacteristicNumbers::arrheniusNumber")

    @property
    def atomicAttenuationCoefficient(self):
        return self._model.element("ISQAtomicNuclear::atomicAttenuationCoefficient")

    @property
    def atomicMass(self):
        return self._model.element("ISQAtomicNuclear::atomicMass")

    @property
    def atomicNumber(self):
        return self._model.element("ISQAtomicNuclear::atomicNumber")

    @property
    def atomicScatteringFactor(self):
        return self._model.element("ISQCondensedMatter::atomicScatteringFactor")

    @property
    def attenuation(self):
        return self._model.element("ISQSpaceTime::attenuation")

    @property
    def atwoodNumber(self):
        return self._model.element("ISQCharacteristicNumbers::atwoodNumber")

    @property
    def averageEnergyLossPerElementaryChargeProduced(self):
        return self._model.element("ISQAtomicNuclear::averageEnergyLossPerElementaryChargeProduced")

    @property
    def averageInformationRate(self):
        return self._model.element("ISQInformation::averageInformationRate")

    @property
    def averageLogarithmicEnergyDecrement(self):
        return self._model.element("ISQAtomicNuclear::averageLogarithmicEnergyDecrement")

    @property
    def averageTransinformationRate(self):
        return self._model.element("ISQInformation::averageTransinformationRate")

    @property
    def azimuth(self):
        return self._model.element("ISQSpaceTime::SphericalPosition3dVector::azimuth")

    @property
    def azimuthUnit(self):
        return self._model.element("ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit")

    @property
    def bagnoldNumber(self):
        return self._model.element("ISQCharacteristicNumbers::bagnoldNumber")

    @property
    def bagnoldNumberForSolidParticles(self):
        return self._model.element("ISQCharacteristicNumbers::bagnoldNumberForSolidParticles")

    @property
    def baseQuantities(self):
        return self._model.element("ISQBase::'International System of Quantities'::baseQuantities")

    @property
    def batchelorNumber(self):
        return self._model.element("ISQCharacteristicNumbers::batchelorNumber")

    @property
    def bejanNumber(self):
        return self._model.element("ISQCharacteristicNumbers::bejanNumber")

    @property
    def bejanNumberForEntropy(self):
        return self._model.element("ISQCharacteristicNumbers::bejanNumberForEntropy")

    @property
    def bejanNumberForHeatTransfer(self):
        return self._model.element("ISQCharacteristicNumbers::bejanNumberForHeatTransfer")

    @property
    def bejanNumberForMassTransfer(self):
        return self._model.element("ISQCharacteristicNumbers::bejanNumberForMassTransfer")

    @property
    def betaDisintegrationEnergy(self):
        return self._model.element("ISQAtomicNuclear::betaDisintegrationEnergy")

    @property
    def binaryDigitRate(self):
        return self._model.element("ISQInformation::binaryDigitRate")

    @property
    def bindingFraction(self):
        return self._model.element("ISQAtomicNuclear::bindingFraction")

    @property
    def binghamNumber(self):
        return self._model.element("ISQCharacteristicNumbers::binghamNumber")

    @property
    def biotNumber(self):
        return self._model.element("ISQCharacteristicNumbers::biotNumber")

    @property
    def biotNumberForMassTransfer(self):
        return self._model.element("ISQCharacteristicNumbers::biotNumberForMassTransfer")

    @property
    def blakeNumber(self):
        return self._model.element("ISQCharacteristicNumbers::blakeNumber")

    @property
    def bodensteinNumber(self):
        return self._model.element("ISQCharacteristicNumbers::bodensteinNumber")

    @property
    def bohrMagneton(self):
        return self._model.element("ISQAtomicNuclear::bohrMagneton")

    @property
    def bohrRadius(self):
        return self._model.element("ISQAtomicNuclear::bohrRadius")

    @property
    def boltzmannNumber(self):
        return self._model.element("ISQCharacteristicNumbers::boltzmannNumber")

    @property
    def bondNumber(self):
        return self._model.element("ISQCharacteristicNumbers::bondNumber")

    @property
    def braggAngle(self):
        return self._model.element("ISQCondensedMatter::braggAngle")

    @property
    def brinkmanNumber(self):
        return self._model.element("ISQCharacteristicNumbers::brinkmanNumber")

    @property
    def callIntensity(self):
        return self._model.element("ISQInformation::callIntensity")

    @property
    def canonicalPartitionFunction(self):
        return self._model.element("ISQChemistryMolecular::canonicalPartitionFunction")

    @property
    def capacitance(self):
        return self._model.element("ISQElectromagnetism::capacitance")

    @property
    def capillaryNumber(self):
        return self._model.element("ISQCharacteristicNumbers::capillaryNumber")

    @property
    def carnotNumber(self):
        return self._model.element("ISQCharacteristicNumbers::carnotNumber")

    @property
    def carrierLifetime(self):
        return self._model.element("ISQCondensedMatter::carrierLifetime")

    @property
    def carrierPower(self):
        return self._model.element("ISQInformation::carrierPower")

    @property
    def cartesianAcceleration3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianAcceleration3dVector")

    @property
    def cartesianAngularAcceleration3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianAngularAcceleration3dVector")

    @property
    def cartesianAngularImpulse3dVector(self):
        return self._model.element("ISQMechanics::cartesianAngularImpulse3dVector")

    @property
    def cartesianAngularMomentum3dVector(self):
        return self._model.element("ISQMechanics::cartesianAngularMomentum3dVector")

    @property
    def cartesianAngularReciprocalLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianAngularReciprocalLattice3dVector")

    @property
    def cartesianAngularVelocity3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianAngularVelocity3dVector")

    @property
    def cartesianBurgers3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianBurgers3dVector")

    @property
    def cartesianDisplacement3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianDisplacement3dVector")

    @property
    def cartesianDisplacementCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianDisplacementCurrentDensity3dVector")

    @property
    def cartesianDragForce3dVector(self):
        return self._model.element("ISQMechanics::cartesianDragForce3dVector")

    @property
    def cartesianElectricCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianElectricCurrentDensity3dVector")

    @property
    def cartesianElectricDipoleMoment3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianElectricDipoleMoment3dVector")

    @property
    def cartesianElectricFieldStrength3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianElectricFieldStrength3dVector")

    @property
    def cartesianElectricFluxDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianElectricFluxDensity3dVector")

    @property
    def cartesianElectricPolarization3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianElectricPolarization3dVector")

    @property
    def cartesianEquilibriumPosition3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianEquilibriumPosition3dVector")

    @property
    def cartesianForce3dVector(self):
        return self._model.element("ISQMechanics::cartesianForce3dVector")

    @property
    def cartesianFundamentalLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianFundamentalLattice3dVector")

    @property
    def cartesianFundamentalReciprocalLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianFundamentalReciprocalLattice3dVector")

    @property
    def cartesianImpulse3dVector(self):
        return self._model.element("ISQMechanics::cartesianImpulse3dVector")

    @property
    def cartesianKineticFrictionForce3dVector(self):
        return self._model.element("ISQMechanics::cartesianKineticFrictionForce3dVector")

    @property
    def cartesianLattice3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianLattice3dVector")

    @property
    def cartesianLinearElectricCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianLinearElectricCurrentDensity3dVector")

    @property
    def cartesianMagneticDipoleMoment3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagneticDipoleMoment3dVector")

    @property
    def cartesianMagneticFieldStrength3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagneticFieldStrength3dVector")

    @property
    def cartesianMagneticFluxDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagneticFluxDensity3dVector")

    @property
    def cartesianMagneticMoment3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagneticMoment3dVector")

    @property
    def cartesianMagneticPolarization3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagneticPolarization3dVector")

    @property
    def cartesianMagneticVectorPotential3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagneticVectorPotential3dVector")

    @property
    def cartesianMagnetization3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianMagnetization3dVector")

    @property
    def cartesianMassFlow3dVector(self):
        return self._model.element("ISQMechanics::cartesianMassFlow3dVector")

    @property
    def cartesianMomentOfForce3dVector(self):
        return self._model.element("ISQMechanics::cartesianMomentOfForce3dVector")

    @property
    def cartesianMomentum3dVector(self):
        return self._model.element("ISQMechanics::cartesianMomentum3dVector")

    @property
    def cartesianParticleCurrentDensity3dVector(self):
        return self._model.element("ISQAtomicNuclear::cartesianParticleCurrentDensity3dVector")

    @property
    def cartesianParticlePosition3dVector(self):
        return self._model.element("ISQCondensedMatter::cartesianParticlePosition3dVector")

    @property
    def cartesianPosition3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianPosition3dVector")

    @property
    def cartesianPoynting3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianPoynting3dVector")

    @property
    def cartesianRollingResistance3dVector(self):
        return self._model.element("ISQMechanics::cartesianRollingResistance3dVector")

    @property
    def cartesianSoundIntensity3dVector(self):
        return self._model.element("ISQAcoustics::cartesianSoundIntensity3dVector")

    @property
    def cartesianSoundParticleAcceleration3dVector(self):
        return self._model.element("ISQAcoustics::cartesianSoundParticleAcceleration3dVector")

    @property
    def cartesianSoundParticleDisplacement3dVector(self):
        return self._model.element("ISQAcoustics::cartesianSoundParticleDisplacement3dVector")

    @property
    def cartesianSoundParticleVelocity3dVector(self):
        return self._model.element("ISQAcoustics::cartesianSoundParticleVelocity3dVector")

    @property
    def cartesianSpin3dVector(self):
        return self._model.element("ISQAtomicNuclear::cartesianSpin3dVector")

    @property
    def cartesianStaticFrictionForce3dVector(self):
        return self._model.element("ISQMechanics::cartesianStaticFrictionForce3dVector")

    @property
    def cartesianTotalAngularMomentum3dVector(self):
        return self._model.element("ISQAtomicNuclear::cartesianTotalAngularMomentum3dVector")

    @property
    def cartesianTotalCurrentDensity3dVector(self):
        return self._model.element("ISQElectromagnetism::cartesianTotalCurrentDensity3dVector")

    @property
    def cartesianVelocity3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianVelocity3dVector")

    @property
    def cartesianWave3dVector(self):
        return self._model.element("ISQSpaceTime::cartesianWave3dVector")

    @property
    def cartesianWeight3dVector(self):
        return self._model.element("ISQMechanics::cartesianWeight3dVector")

    @property
    def cauchyNumber(self):
        return self._model.element("ISQCharacteristicNumbers::cauchyNumber")

    @property
    def cavitationNumber(self):
        return self._model.element("ISQCharacteristicNumbers::cavitationNumber")

    @property
    def celsiusTemperature(self):
        return self._model.element("ISQThermodynamics::celsiusTemperature")

    @property
    def chandrasekharNumber(self):
        return self._model.element("ISQCharacteristicNumbers::chandrasekharNumber")

    @property
    def channelCapacityPerCharacter(self):
        return self._model.element("ISQInformation::channelCapacityPerCharacter")

    @property
    def channelTimeCapacity(self):
        return self._model.element("ISQInformation::channelTimeCapacity")

    @property
    def characterMeanEntropy(self):
        return self._model.element("ISQInformation::characterMeanEntropy")

    @property
    def characterMeanTransinformationContent(self):
        return self._model.element("ISQInformation::characterMeanTransinformationContent")

    @property
    def characteristicImpedanceOfAMediumForLongitudinalWaves(self):
        return self._model.element("ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves")

    @property
    def chargeNumber(self):
        return self._model.element("ISQAtomicNuclear::chargeNumber")

    @property
    def chemicalPotential(self):
        return self._model.element("ISQChemistryMolecular::chemicalPotential")

    @property
    def chromaticityCoordinatesInTheCie1931StandardColorimetricSystem(self):
        return self._model.element("ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem")

    @property
    def chromaticityCoordinatesInTheCie1964StandardColorimetricSystem(self):
        return self._model.element("ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem")

    @property
    def cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver(self):
        return self._model.element("ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver")

    @property
    def cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver(self):
        return self._model.element("ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver")

    @property
    def clausiusNumber(self):
        return self._model.element("ISQCharacteristicNumbers::clausiusNumber")

    @property
    def clockFrequency(self):
        return self._model.element("ISQInformation::clockFrequency")

    @property
    def coefficientOfHeatTransfer(self):
        return self._model.element("ISQThermodynamics::coefficientOfHeatTransfer")

    @property
    def coercivity(self):
        return self._model.element("ISQElectromagnetism::coercivity")

    @property
    def coherenceLength(self):
        return self._model.element("ISQCondensedMatter::coherenceLength")

    @property
    def colourTemperature(self):
        return self._model.element("ISQLight::colourTemperature")

    @property
    def completedCallIntensity(self):
        return self._model.element("ISQInformation::completedCallIntensity")

    @property
    def complexPower(self):
        return self._model.element("ISQElectromagnetism::complexPower")

    @property
    def compressibility(self):
        return self._model.element("ISQMechanics::compressibility")

    @property
    def compressibilityNumber(self):
        return self._model.element("ISQCharacteristicNumbers::compressibilityNumber")

    @property
    def comptonWavelength(self):
        return self._model.element("ISQAtomicNuclear::comptonWavelength")

    @property
    def conditionalEntropy(self):
        return self._model.element("ISQInformation::conditionalEntropy")

    @property
    def conditionalInformationContent(self):
        return self._model.element("ISQInformation::conditionalInformationContent")

    @property
    def conductance(self):
        return self._model.element("ISQElectromagnetism::conductance")

    @property
    def conductanceForAlternatingCurrent(self):
        return self._model.element("ISQElectromagnetism::conductanceForAlternatingCurrent")

    @property
    def conductivity(self):
        return self._model.element("ISQElectromagnetism::conductivity")

    @property
    def correlatedColourTemperature(self):
        return self._model.element("ISQLight::correlatedColourTemperature")

    @property
    def couplingFactor(self):
        return self._model.element("ISQElectromagnetism::couplingFactor")

    @property
    def cowlingNumber(self):
        return self._model.element("ISQCharacteristicNumbers::cowlingNumber")

    @property
    def crossSection(self):
        return self._model.element("ISQAtomicNuclear::crossSection")

    @property
    def cubicExpansionCoefficient(self):
        return self._model.element("ISQThermodynamics::cubicExpansionCoefficient")

    @property
    def curieTemperature(self):
        return self._model.element("ISQCondensedMatter::curieTemperature")

    @property
    def currentLinkage(self):
        return self._model.element("ISQElectromagnetism::currentLinkage")

    @property
    def curvature(self):
        return self._model.element("ISQSpaceTime::curvature")

    @property
    def cyclotronAngularFrequency(self):
        return self._model.element("ISQAtomicNuclear::cyclotronAngularFrequency")

    @property
    def cylindricalDisplacement3dVector(self):
        return self._model.element("ISQSpaceTime::cylindricalDisplacement3dVector")

    @property
    def cylindricalPosition3dVector(self):
        return self._model.element("ISQSpaceTime::cylindricalPosition3dVector")

    @property
    def dampingCoefficient(self):
        return self._model.element("ISQSpaceTime::dampingCoefficient")

    @property
    def darcyFrictionFactor(self):
        return self._model.element("ISQCharacteristicNumbers::darcyFrictionFactor")

    @property
    def deanNumber(self):
        return self._model.element("ISQCharacteristicNumbers::deanNumber")

    @property
    def deborahNumber(self):
        return self._model.element("ISQCharacteristicNumbers::deborahNumber")

    @property
    def debyeAngularFrequency(self):
        return self._model.element("ISQCondensedMatter::debyeAngularFrequency")

    @property
    def debyeAngularWavenumber(self):
        return self._model.element("ISQCondensedMatter::debyeAngularWavenumber")

    @property
    def debyeTemperature(self):
        return self._model.element("ISQCondensedMatter::debyeTemperature")

    @property
    def debyeWallerFactor(self):
        return self._model.element("ISQCondensedMatter::debyeWallerFactor")

    @property
    def decayConstant(self):
        return self._model.element("ISQAtomicNuclear::decayConstant")

    @property
    def decisionContent(self):
        return self._model.element("ISQInformation::decisionContent")

    @property
    def degeneracy(self):
        return self._model.element("ISQChemistryMolecular::degeneracy")

    @property
    def degreeOfDissociation(self):
        return self._model.element("ISQChemistryMolecular::degreeOfDissociation")

    @property
    def densityOfHeatFlowRate(self):
        return self._model.element("ISQThermodynamics::densityOfHeatFlowRate")

    @property
    def densityOfVibrationalStates(self):
        return self._model.element("ISQCondensedMatter::densityOfVibrationalStates")

    @property
    def dewPointTemperature(self):
        return self._model.element("ISQThermodynamics::dewPointTemperature")

    @property
    def diameter(self):
        return self._model.element("ISQSpaceTime::diameter")

    @property
    def diffusionArea(self):
        return self._model.element("ISQAtomicNuclear::diffusionArea")

    @property
    def diffusionCoefficient(self):
        return self._model.element("ISQChemistryMolecular::diffusionCoefficient")

    @property
    def diffusionCoefficientForFluenceRate(self):
        return self._model.element("ISQAtomicNuclear::diffusionCoefficientForFluenceRate")

    @property
    def diffusionLength(self):
        return self._model.element("ISQAtomicNuclear::diffusionLength")

    @property
    def diffusionLengthForCondensedMatterPhysics(self):
        return self._model.element("ISQCondensedMatter::diffusionLengthForCondensedMatterPhysics")

    @property
    def dimensions(self):
        return self._model.element("ISQMechanics::Cartesian3dStressMeasurementReference::dimensions")

    @property
    def directionAndEnergyDistributionOfCrossSection(self):
        return self._model.element("ISQAtomicNuclear::directionAndEnergyDistributionOfCrossSection")

    @property
    def directionDistributionOfCrossSection(self):
        return self._model.element("ISQAtomicNuclear::directionDistributionOfCrossSection")

    @property
    def displacement3dVector(self):
        return self._model.element("ISQSpaceTime::displacement3dVector")

    @property
    def displacementCurrent(self):
        return self._model.element("ISQElectromagnetism::displacementCurrent")

    @property
    def displacementCurrentDensity(self):
        return self._model.element("ISQElectromagnetism::displacementCurrentDensity")

    @property
    def distance(self):
        return self._model.element("ISQSpaceTime::distance")

    @property
    def donorDensity(self):
        return self._model.element("ISQCondensedMatter::donorDensity")

    @property
    def doseEquivalent(self):
        return self._model.element("ISQAtomicNuclear::doseEquivalent")

    @property
    def doseEquivalentRate(self):
        return self._model.element("ISQAtomicNuclear::doseEquivalentRate")

    @property
    def dragCoefficient(self):
        return self._model.element("ISQMechanics::dragCoefficient")

    @property
    def duration(self):
        return self._model.element("ISQBase::duration")

    @property
    def durationPF(self):
        return self._model.element("ISQThermodynamics::ThermalResistanceUnit::durationPF")

    @property
    def dynamicCapillaryNumber(self):
        return self._model.element("ISQCharacteristicNumbers::dynamicCapillaryNumber")

    @property
    def dynamicViscosity(self):
        return self._model.element("ISQMechanics::dynamicViscosity")

    @property
    def eckertNumber(self):
        return self._model.element("ISQCharacteristicNumbers::eckertNumber")

    @property
    def effectiveMass(self):
        return self._model.element("ISQCondensedMatter::effectiveMass")

    @property
    def ekmanNumber(self):
        return self._model.element("ISQCharacteristicNumbers::ekmanNumber")

    @property
    def elasticityNumber(self):
        return self._model.element("ISQCharacteristicNumbers::elasticityNumber")

    @property
    def electricCharge(self):
        return self._model.element("ISQElectromagnetism::electricCharge")

    @property
    def electricChargeDensity(self):
        return self._model.element("ISQElectromagnetism::electricChargeDensity")

    @property
    def electricConstant(self):
        return self._model.element("ISQElectromagnetism::electricConstant")

    @property
    def electricCurrent(self):
        return self._model.element("ISQBase::electricCurrent")

    @property
    def electricCurrentDensity(self):
        return self._model.element("ISQElectromagnetism::electricCurrentDensity")

    @property
    def electricCurrentPF(self):
        return self._model.element("ISQElectromagnetism::TotalCurrentDensityUnit::electricCurrentPF")

    @property
    def electricCurrentPhasor(self):
        return self._model.element("ISQElectromagnetism::electricCurrentPhasor")

    @property
    def electricDipoleMoment(self):
        return self._model.element("ISQElectromagnetism::electricDipoleMoment")

    @property
    def electricFieldParameter(self):
        return self._model.element("ISQCharacteristicNumbers::electricFieldParameter")

    @property
    def electricFieldStrength(self):
        return self._model.element("ISQElectromagnetism::electricFieldStrength")

    @property
    def electricFlux(self):
        return self._model.element("ISQElectromagnetism::electricFlux")

    @property
    def electricFluxDensity(self):
        return self._model.element("ISQElectromagnetism::electricFluxDensity")

    @property
    def electricPolarization(self):
        return self._model.element("ISQElectromagnetism::electricPolarization")

    @property
    def electricPotential(self):
        return self._model.element("ISQElectromagnetism::electricPotential")

    @property
    def electricPotentialDifference(self):
        return self._model.element("ISQElectromagnetism::electricPotentialDifference")

    @property
    def electricPower(self):
        return self._model.element("ISQElectromagnetism::electricPower")

    @property
    def electricSusceptibility(self):
        return self._model.element("ISQElectromagnetism::electricSusceptibility")

    @property
    def electrolyticConductivity(self):
        return self._model.element("ISQChemistryMolecular::electrolyticConductivity")

