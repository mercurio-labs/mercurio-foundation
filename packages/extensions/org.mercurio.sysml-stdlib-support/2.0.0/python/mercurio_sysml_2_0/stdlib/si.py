from __future__ import annotations


class SINamespace:
    def __init__(self, model):
        self._model = model

    @property
    def ISO_IEC_80000_International_System_of_Units(self):
        return self._model.element("SI::'ISO/IEC 80000 International System of Units'")

    @property
    def ampere(self):
        return self._model.element("SI::ampere")

    @property
    def ampere_metre_squared(self):
        return self._model.element("SI::'ampere metre squared'")

    @property
    def ampere_metre_squared_joule_to_the_power_minus_1_second_to_the_power_minus_1(self):
        return self._model.element("SI::'ampere metre squared joule to the power minus 1 second to the power minus 1'")

    @property
    def ampere_metre_to_the_power_minus_2_kelvin_to_the_power_minus_2(self):
        return self._model.element("SI::'ampere metre to the power minus 2 kelvin to the power minus 2'")

    @property
    def ampere_per_metre(self):
        return self._model.element("SI::'ampere per metre'")

    @property
    def ampere_per_square_metre(self):
        return self._model.element("SI::'ampere per square metre'")

    @property
    def ampere_second_per_kilogram(self):
        return self._model.element("SI::'ampere second per kilogram'")

    @property
    def astronomical_unit(self):
        return self._model.element("SI::'astronomical unit'")

    @property
    def atomic_mass_unit(self):
        return self._model.element("SI::'atomic mass unit'")

    @property
    def barn(self):
        return self._model.element("SI::barn")

    @property
    def baseUnits(self):
        return self._model.element("SI::'ISO/IEC 80000 International System of Units'::baseUnits")

    @property
    def baud(self):
        return self._model.element("SI::baud")

    @property
    def becquerel(self):
        return self._model.element("SI::becquerel")

    @property
    def becquerel_per_cubic_metre(self):
        return self._model.element("SI::'becquerel per cubic metre'")

    @property
    def becquerel_per_kilogram(self):
        return self._model.element("SI::'becquerel per kilogram'")

    @property
    def becquerel_per_square_metre(self):
        return self._model.element("SI::'becquerel per square metre'")

    @property
    def bit(self):
        return self._model.element("SI::bit")

    @property
    def bit_per_second(self):
        return self._model.element("SI::'bit per second'")

    @property
    def byte(self):
        return self._model.element("SI::byte")

    @property
    def byte_per_second(self):
        return self._model.element("SI::'byte per second'")

    @property
    def candela(self):
        return self._model.element("SI::candela")

    @property
    def candela_metre_to_the_power_minus_2(self):
        return self._model.element("SI::'candela metre to the power minus 2'")

    @property
    def candela_steradian(self):
        return self._model.element("SI::'candela steradian'")

    @property
    def candela_steradian_kilogram_to_the_power_minus_1_metre_to_the_power_minus_2_second_to_the_power_3(self):
        return self._model.element("SI::'candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3'")

    @property
    def candela_steradian_metre_to_the_power_minus_2(self):
        return self._model.element("SI::'candela steradian metre to the power minus 2'")

    @property
    def candela_steradian_metre_to_the_power_minus_2_second(self):
        return self._model.element("SI::'candela steradian metre to the power minus 2 second'")

    @property
    def candela_steradian_second(self):
        return self._model.element("SI::'candela steradian second'")

    @property
    def celsiusToKelvinScaleMapping(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::celsiusToKelvinScaleMapping")

    @property
    def centimetre(self):
        return self._model.element("SI::centimetre")

    @property
    def conversionFactor(self):
        return self._model.element("SI::tonne::unitConversion::conversionFactor")

    @property
    def coulomb(self):
        return self._model.element("SI::coulomb")

    @property
    def coulomb_metre(self):
        return self._model.element("SI::'coulomb metre'")

    @property
    def coulomb_per_cubic_metre(self):
        return self._model.element("SI::'coulomb per cubic metre'")

    @property
    def coulomb_per_kilogram(self):
        return self._model.element("SI::'coulomb per kilogram'")

    @property
    def coulomb_per_kilogram_second(self):
        return self._model.element("SI::'coulomb per kilogram second'")

    @property
    def coulomb_per_metre(self):
        return self._model.element("SI::'coulomb per metre'")

    @property
    def coulomb_per_square_metre(self):
        return self._model.element("SI::'coulomb per square metre'")

    @property
    def dalton(self):
        return self._model.element("SI::dalton")

    @property
    def day(self):
        return self._model.element("SI::day")

    @property
    def decade(self):
        return self._model.element("SI::decade")

    @property
    def decibel(self):
        return self._model.element("SI::decibel")

    @property
    def definition(self):
        return self._model.element("SI::kelvin::temperatureOfWaterAtTriplePointInK::definition")

    @property
    def definitionalQuantityValues(self):
        return self._model.element("SI::kelvin::definitionalQuantityValues")

    @property
    def degree(self):
        return self._model.element("SI::degree")

    @property
    def degree_celsius_absolute_temperature_scale(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'")

    @property
    def degree_celsius_temperature_difference(self):
        return self._model.element("SI::'degree celsius (temperature difference)'")

    @property
    def electronvolt(self):
        return self._model.element("SI::electronvolt")

    @property
    def electronvolt_joule_kilogram_metre_squared_second_to_the_power_minus_2(self):
        return self._model.element("SI::'electronvolt joule kilogram metre squared second to the power minus 2'")

    @property
    def electronvolt_metre_to_the_power_minus_2_per_kilogram(self):
        return self._model.element("SI::'electronvolt metre to the power minus 2 per kilogram'")

    @property
    def electronvolt_per_metre(self):
        return self._model.element("SI::'electronvolt per metre'")

    @property
    def electronvolt_per_square_metre(self):
        return self._model.element("SI::'electronvolt per square metre'")

    @property
    def erlang(self):
        return self._model.element("SI::erlang")

    @property
    def farad(self):
        return self._model.element("SI::farad")

    @property
    def farad_per_metre(self):
        return self._model.element("SI::'farad per metre'")

    @property
    def g_per_l(self):
        return self._model.element("SI::'g per l'")

    @property
    def g_per_mole(self):
        return self._model.element("SI::'g per mole'")

    @property
    def gigajoule(self):
        return self._model.element("SI::gigajoule")

    @property
    def gram(self):
        return self._model.element("SI::gram")

    @property
    def gray(self):
        return self._model.element("SI::gray")

    @property
    def gray_per_second(self):
        return self._model.element("SI::'gray per second'")

    @property
    def hartley(self):
        return self._model.element("SI::hartley")

    @property
    def hartley_per_second(self):
        return self._model.element("SI::'hartley per second'")

    @property
    def henry(self):
        return self._model.element("SI::henry")

    @property
    def henry_per_metre(self):
        return self._model.element("SI::'henry per metre'")

    @property
    def henry_to_the_power_minus_1(self):
        return self._model.element("SI::'henry to the power minus 1'")

    @property
    def hertz(self):
        return self._model.element("SI::hertz")

    @property
    def hour(self):
        return self._model.element("SI::hour")

    @property
    def isExact(self):
        return self._model.element("SI::electronvolt::unitConversion::isExact")

    @property
    def joule(self):
        return self._model.element("SI::joule")

    @property
    def joule_metre_squared_per_kilogram(self):
        return self._model.element("SI::'joule metre squared per kilogram'")

    @property
    def joule_per_cubic_metre(self):
        return self._model.element("SI::'joule per cubic metre'")

    @property
    def joule_per_cubic_metre_nm(self):
        return self._model.element("SI::'joule per cubic metre nm'")

    @property
    def joule_per_kelvin(self):
        return self._model.element("SI::'joule per kelvin'")

    @property
    def joule_per_kilogram(self):
        return self._model.element("SI::'joule per kilogram'")

    @property
    def joule_per_kilogram_kelvin(self):
        return self._model.element("SI::'joule per kilogram kelvin'")

    @property
    def joule_per_metre(self):
        return self._model.element("SI::'joule per metre'")

    @property
    def joule_per_mole(self):
        return self._model.element("SI::'joule per mole'")

    @property
    def joule_per_mole_kelvin(self):
        return self._model.element("SI::'joule per mole kelvin'")

    @property
    def joule_per_nm(self):
        return self._model.element("SI::'joule per nm'")

    @property
    def joule_per_second(self):
        return self._model.element("SI::'joule per second'")

    @property
    def joule_per_square_metre(self):
        return self._model.element("SI::'joule per square metre'")

    @property
    def joule_per_square_metre_nm(self):
        return self._model.element("SI::'joule per square metre nm'")

    @property
    def joule_second(self):
        return self._model.element("SI::'joule second'")

    @property
    def joule_second_electronvolt_second(self):
        return self._model.element("SI::'joule second electronvolt second'")

    @property
    def joule_second_to_the_power_minus_1(self):
        return self._model.element("SI::'joule second to the power minus 1'")

    @property
    def joule_to_the_power_minus_1_metre_to_the_power_minus_3_electronvolt_to_the_power_minus_1_metre_to_the_power_minus_3(self):
        return self._model.element("SI::'joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3'")

    @property
    def kelvin(self):
        return self._model.element("SI::kelvin")

    @property
    def kelvin_per_pascal(self):
        return self._model.element("SI::'kelvin per pascal'")

    @property
    def kelvin_per_watt(self):
        return self._model.element("SI::'kelvin per watt'")

    @property
    def kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kelvin to the power minus 1'")

    @property
    def kilogram(self):
        return self._model.element("SI::kilogram")

    @property
    def kilogram_metre_cubed_second_to_the_power_minus_3_ampere_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram metre cubed second to the power minus 3 ampere to the power minus 2'")

    @property
    def kilogram_metre_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre second to the power minus 1'")

    @property
    def kilogram_metre_second_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram metre second to the power minus 2'")

    @property
    def kilogram_metre_second_to_the_power_minus_3(self):
        return self._model.element("SI::'kilogram metre second to the power minus 3'")

    @property
    def kilogram_metre_second_to_the_power_minus_3_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre second to the power minus 3 kelvin to the power minus 1'")

    @property
    def kilogram_metre_second_to_the_power_minus_3_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre second to the power minus 3 steradian to the power minus 1'")

    @property
    def kilogram_metre_squared(self):
        return self._model.element("SI::'kilogram metre squared'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 2'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_2_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 2 kelvin to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_2_kelvin_to_the_power_minus_1_mole_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_2_mole_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 2 mole to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_3(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 3'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_3_ampere_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 3 ampere to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_3_ampere_to_the_power_minus_1_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_3_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 3 kelvin to the power minus 1'")

    @property
    def kilogram_metre_squared_second_to_the_power_minus_3_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre squared second to the power minus 3 steradian to the power minus 1'")

    @property
    def kilogram_metre_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre to the power minus 1'")

    @property
    def kilogram_metre_to_the_power_minus_1_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre to the power minus 1 second to the power minus 1'")

    @property
    def kilogram_metre_to_the_power_minus_1_second_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram metre to the power minus 1 second to the power minus 2'")

    @property
    def kilogram_metre_to_the_power_minus_1_second_to_the_power_minus_2_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1'")

    @property
    def kilogram_metre_to_the_power_minus_1_second_to_the_power_minus_3(self):
        return self._model.element("SI::'kilogram metre to the power minus 1 second to the power minus 3'")

    @property
    def kilogram_metre_to_the_power_minus_1_second_to_the_power_minus_3_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1'")

    @property
    def kilogram_metre_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram metre to the power minus 2'")

    @property
    def kilogram_metre_to_the_power_minus_2_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre to the power minus 2 second to the power minus 1'")

    @property
    def kilogram_metre_to_the_power_minus_2_second_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram metre to the power minus 2 second to the power minus 2'")

    @property
    def kilogram_metre_to_the_power_minus_3(self):
        return self._model.element("SI::'kilogram metre to the power minus 3'")

    @property
    def kilogram_metre_to_the_power_minus_4_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram metre to the power minus 4 second to the power minus 1'")

    @property
    def kilogram_mole_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram mole to the power minus 1'")

    @property
    def kilogram_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram second to the power minus 1'")

    @property
    def kilogram_second_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram second to the power minus 2'")

    @property
    def kilogram_second_to_the_power_minus_2_ampere_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram second to the power minus 2 ampere to the power minus 1'")

    @property
    def kilogram_second_to_the_power_minus_3(self):
        return self._model.element("SI::'kilogram second to the power minus 3'")

    @property
    def kilogram_second_to_the_power_minus_3_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram second to the power minus 3 kelvin to the power minus 1'")

    @property
    def kilogram_second_to_the_power_minus_3_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram second to the power minus 3 steradian to the power minus 1'")

    @property
    def kilogram_to_the_power_2_metre_to_the_power_4_second_to_the_power_minus_6_ampere_to_the_power_minus_2_kelvin_to_the_power_minus_2(self):
        return self._model.element("SI::'kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2'")

    @property
    def kilogram_to_the_power_2_metre_to_the_power_minus_2_second_to_the_power_minus_3(self):
        return self._model.element("SI::'kilogram to the power 2 metre to the power minus 2 second to the power minus 3'")

    @property
    def kilogram_to_the_power_minus_1_ampere(self):
        return self._model.element("SI::'kilogram to the power minus 1 ampere'")

    @property
    def kilogram_to_the_power_minus_1_metre_cubed(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre cubed'")

    @property
    def kilogram_to_the_power_minus_1_metre_second_to_the_power_2(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre second to the power 2'")

    @property
    def kilogram_to_the_power_minus_1_metre_second_to_the_power_2_kelvin(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre second to the power 2 kelvin'")

    @property
    def kilogram_to_the_power_minus_1_metre_squared(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre squared'")

    @property
    def kilogram_to_the_power_minus_1_metre_to_the_power_minus_2_second_to_the_power_3_kelvin(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin'")

    @property
    def kilogram_to_the_power_minus_1_metre_to_the_power_minus_3_second_to_the_power_3_ampere_to_the_power_2(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2'")

    @property
    def kilogram_to_the_power_minus_1_metre_to_the_power_minus_5_second_to_the_power_2(self):
        return self._model.element("SI::'kilogram to the power minus 1 metre to the power minus 5 second to the power 2'")

    @property
    def kilogram_to_the_power_minus_1_second_ampere(self):
        return self._model.element("SI::'kilogram to the power minus 1 second ampere'")

    @property
    def kilogram_to_the_power_minus_1_second_to_the_power_2(self):
        return self._model.element("SI::'kilogram to the power minus 1 second to the power 2'")

    @property
    def kilogram_to_the_power_minus_1_second_to_the_power_2_ampere(self):
        return self._model.element("SI::'kilogram to the power minus 1 second to the power 2 ampere'")

    @property
    def kilogram_to_the_power_minus_1_second_to_the_power_3_ampere_to_the_power_2_mole_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1'")

    @property
    def kilogram_to_the_power_minus_1_second_to_the_power_3_kelvin(self):
        return self._model.element("SI::'kilogram to the power minus 1 second to the power 3 kelvin'")

    @property
    def kilogram_to_the_power_minus_1_second_to_the_power_minus_1(self):
        return self._model.element("SI::'kilogram to the power minus 1 second to the power minus 1'")

    @property
    def kilojoule(self):
        return self._model.element("SI::kilojoule")

    @property
    def kilometre(self):
        return self._model.element("SI::kilometre")

    @property
    def kilometre_per_hour(self):
        return self._model.element("SI::'kilometre per hour'")

    @property
    def kilowatt(self):
        return self._model.element("SI::kilowatt")

    @property
    def litre(self):
        return self._model.element("SI::litre")

    @property
    def lumen(self):
        return self._model.element("SI::lumen")

    @property
    def lumen_per_square_metre(self):
        return self._model.element("SI::'lumen per square metre'")

    @property
    def lumen_per_watt(self):
        return self._model.element("SI::'lumen per watt'")

    @property
    def lumen_second(self):
        return self._model.element("SI::'lumen second'")

    @property
    def lux(self):
        return self._model.element("SI::lux")

    @property
    def lux_second(self):
        return self._model.element("SI::'lux second'")

    @property
    def mappedQuantityValue(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::celsiusToKelvinScaleMapping::mappedQuantityValue")

    @property
    def megajoule(self):
        return self._model.element("SI::megajoule")

    @property
    def metre(self):
        return self._model.element("SI::metre")

    @property
    def metre_cubed(self):
        return self._model.element("SI::'metre cubed'")

    @property
    def metre_cubed_mole_to_the_power_minus_1(self):
        return self._model.element("SI::'metre cubed mole to the power minus 1'")

    @property
    def metre_cubed_per_coulomb_cubic_metre_second_to_the_power_minus_1_ampere_to_the_power_minus_1(self):
        return self._model.element("SI::'metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1'")

    @property
    def metre_cubed_second_to_the_power_minus_1(self):
        return self._model.element("SI::'metre cubed second to the power minus 1'")

    @property
    def metre_per_second(self):
        return self._model.element("SI::'metre per second'")

    @property
    def metre_second_to_the_power_minus_1(self):
        return self._model.element("SI::'metre second to the power minus 1'")

    @property
    def metre_second_to_the_power_minus_2(self):
        return self._model.element("SI::'metre second to the power minus 2'")

    @property
    def metre_squared(self):
        return self._model.element("SI::'metre squared'")

    @property
    def metre_squared_ampere(self):
        return self._model.element("SI::'metre squared ampere'")

    @property
    def metre_squared_kelvin_per_watt(self):
        return self._model.element("SI::'metre squared kelvin per watt'")

    @property
    def metre_squared_mole_to_the_power_minus_1(self):
        return self._model.element("SI::'metre squared mole to the power minus 1'")

    @property
    def metre_squared_per_joule(self):
        return self._model.element("SI::'metre squared per joule'")

    @property
    def metre_squared_per_joule_steradian(self):
        return self._model.element("SI::'metre squared per joule steradian'")

    @property
    def metre_squared_per_volt_second(self):
        return self._model.element("SI::'metre squared per volt second'")

    @property
    def metre_squared_second_to_the_power_minus_1(self):
        return self._model.element("SI::'metre squared second to the power minus 1'")

    @property
    def metre_squared_second_to_the_power_minus_2(self):
        return self._model.element("SI::'metre squared second to the power minus 2'")

    @property
    def metre_squared_second_to_the_power_minus_2_kelvin_to_the_power_minus_1(self):
        return self._model.element("SI::'metre squared second to the power minus 2 kelvin to the power minus 1'")

    @property
    def metre_squared_second_to_the_power_minus_3(self):
        return self._model.element("SI::'metre squared second to the power minus 3'")

    @property
    def metre_squared_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'metre squared steradian to the power minus 1'")

    @property
    def metre_to_the_power_4(self):
        return self._model.element("SI::'metre to the power 4'")

    @property
    def metre_to_the_power_4_second_to_the_power_minus_2(self):
        return self._model.element("SI::'metre to the power 4 second to the power minus 2'")

    @property
    def metre_to_the_power_minus_1(self):
        return self._model.element("SI::'metre to the power minus 1'")

    @property
    def metre_to_the_power_minus_2(self):
        return self._model.element("SI::'metre to the power minus 2'")

    @property
    def metre_to_the_power_minus_2_second_to_the_power_minus_1(self):
        return self._model.element("SI::'metre to the power minus 2 second to the power minus 1'")

    @property
    def metre_to_the_power_minus_2_second_to_the_power_minus_1_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'metre to the power minus 2 second to the power minus 1 steradian to the power minus 1'")

    @property
    def metre_to_the_power_minus_3(self):
        return self._model.element("SI::'metre to the power minus 3'")

    @property
    def metre_to_the_power_minus_3_second(self):
        return self._model.element("SI::'metre to the power minus 3 second'")

    @property
    def metre_to_the_power_minus_3_second_to_the_power_minus_1(self):
        return self._model.element("SI::'metre to the power minus 3 second to the power minus 1'")

    @property
    def millilitre(self):
        return self._model.element("SI::millilitre")

    @property
    def millimetre(self):
        return self._model.element("SI::millimetre")

    @property
    def millinewton(self):
        return self._model.element("SI::millinewton")

    @property
    def minute(self):
        return self._model.element("SI::minute")

    @property
    def minute_angle(self):
        return self._model.element("SI::'minute (angle)'")

    @property
    def ml_per_l(self):
        return self._model.element("SI::'ml per l'")

    @property
    def mole(self):
        return self._model.element("SI::mole")

    @property
    def mole_kilogram_to_the_power_minus_1(self):
        return self._model.element("SI::'mole kilogram to the power minus 1'")

    @property
    def mole_metre_to_the_power_minus_3(self):
        return self._model.element("SI::'mole metre to the power minus 3'")

    @property
    def mole_per_cubic_metre(self):
        return self._model.element("SI::'mole per cubic metre'")

    @property
    def mole_per_kilogram(self):
        return self._model.element("SI::'mole per kilogram'")

    @property
    def mole_per_l(self):
        return self._model.element("SI::'mole per l'")

    @property
    def nanometre(self):
        return self._model.element("SI::nanometre")

    @property
    def natural_unit_of_information(self):
        return self._model.element("SI::'natural unit of information'")

    @property
    def natural_unit_of_information_per_second(self):
        return self._model.element("SI::'natural unit of information per second'")

    @property
    def newton(self):
        return self._model.element("SI::newton")

    @property
    def newton_metre(self):
        return self._model.element("SI::'newton metre'")

    @property
    def newton_metre_second(self):
        return self._model.element("SI::'newton metre second'")

    @property
    def newton_metre_second_to_the_power_minus_1(self):
        return self._model.element("SI::'newton metre second to the power minus 1'")

    @property
    def newton_metre_to_the_power_minus_1(self):
        return self._model.element("SI::'newton metre to the power minus 1'")

    @property
    def newton_metre_to_the_power_minus_2(self):
        return self._model.element("SI::'newton metre to the power minus 2'")

    @property
    def newton_second(self):
        return self._model.element("SI::'newton second'")

    @property
    def ngstr_m(self):
        return self._model.element("SI::'ångström'")

    @property
    def num(self):
        return self._model.element("SI::kelvin::temperatureOfWaterAtTriplePointInK::num")

    @property
    def octave(self):
        return self._model.element("SI::octave")

    @property
    def octet(self):
        return self._model.element("SI::octet")

    @property
    def octet_per_second(self):
        return self._model.element("SI::'octet per second'")

    @property
    def ohm(self):
        return self._model.element("SI::ohm")

    @property
    def ohm_metre(self):
        return self._model.element("SI::'ohm metre'")

    @property
    def origin(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::zeroDegreeCelsiusToKelvinShift::origin")

    @property
    def pascal(self):
        return self._model.element("SI::pascal")

    @property
    def pascal_per_kelvin(self):
        return self._model.element("SI::'pascal per kelvin'")

    @property
    def pascal_second(self):
        return self._model.element("SI::'pascal second'")

    @property
    def pascal_second_per_cubic_metre(self):
        return self._model.element("SI::'pascal second per cubic metre'")

    @property
    def pascal_second_per_metre(self):
        return self._model.element("SI::'pascal second per metre'")

    @property
    def pascal_to_the_power_2_second(self):
        return self._model.element("SI::'pascal to the power 2 second'")

    @property
    def pascal_to_the_power_minus_1(self):
        return self._model.element("SI::'pascal to the power minus 1'")

    @property
    def prefix(self):
        return self._model.element("SI::nanometre::unitConversion::prefix")

    @property
    def quantityDimension(self):
        return self._model.element("SI::kelvin::quantityDimension")

    @property
    def quantityPowerFactors(self):
        return self._model.element("SI::kelvin::quantityDimension::quantityPowerFactors")

    @property
    def quantityValueMapping(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::quantityValueMapping")

    @property
    def radian(self):
        return self._model.element("SI::radian")

    @property
    def radian_metre_squared_per_kilogram_to_the_power_1(self):
        return self._model.element("SI::'radian metre squared per kilogram to the power 1'")

    @property
    def radian_metre_squared_per_mole(self):
        return self._model.element("SI::'radian metre squared per mole'")

    @property
    def radian_per_metre(self):
        return self._model.element("SI::'radian per metre'")

    @property
    def radian_second_to_the_power_minus_1(self):
        return self._model.element("SI::'radian second to the power minus 1'")

    @property
    def radian_second_to_the_power_minus_2(self):
        return self._model.element("SI::'radian second to the power minus 2'")

    @property
    def referenceQuantityValue(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::celsiusToKelvinScaleMapping::referenceQuantityValue")

    @property
    def referenceUnit(self):
        return self._model.element("SI::tonne::unitConversion::referenceUnit")

    @property
    def second(self):
        return self._model.element("SI::second")

    @property
    def second_ampere(self):
        return self._model.element("SI::'second ampere'")

    @property
    def second_angle(self):
        return self._model.element("SI::'second (angle)'")

    @property
    def second_to_the_power_minus_1(self):
        return self._model.element("SI::'second to the power minus 1'")

    @property
    def second_to_the_power_minus_1_steradian_to_the_power_minus_1(self):
        return self._model.element("SI::'second to the power minus 1 steradian to the power minus 1'")

    @property
    def second_to_the_power_minus_2(self):
        return self._model.element("SI::'second to the power minus 2'")

    @property
    def shannon(self):
        return self._model.element("SI::shannon")

    @property
    def shannon_per_second(self):
        return self._model.element("SI::'shannon per second'")

    @property
    def siemens(self):
        return self._model.element("SI::siemens")

    @property
    def siemens_metre_squared_per_mole(self):
        return self._model.element("SI::'siemens metre squared per mole'")

    @property
    def siemens_per_metre(self):
        return self._model.element("SI::'siemens per metre'")

    @property
    def sievert(self):
        return self._model.element("SI::sievert")

    @property
    def sievert_per_second(self):
        return self._model.element("SI::'sievert per second'")

    @property
    def source(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::zeroDegreeCelsiusToKelvinShift::source")

    @property
    def steradian(self):
        return self._model.element("SI::steradian")

    @property
    def systemOfQuantities(self):
        return self._model.element("SI::'ISO/IEC 80000 International System of Units'::systemOfQuantities")

    @property
    def temperatureOfWaterAtTriplePointInK(self):
        return self._model.element("SI::kelvin::temperatureOfWaterAtTriplePointInK")

    @property
    def temperatureWaterAtFreezingPointInC(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::temperatureWaterAtFreezingPointInC")

    @property
    def temperatureWaterAtTriplePointInC(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::temperatureWaterAtTriplePointInC")

    @property
    def tesla(self):
        return self._model.element("SI::tesla")

    @property
    def tonne(self):
        return self._model.element("SI::tonne")

    @property
    def unit(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::unit")

    @property
    def unitConversion(self):
        return self._model.element("SI::tonne::unitConversion")

    @property
    def volt(self):
        return self._model.element("SI::volt")

    @property
    def volt_ampere(self):
        return self._model.element("SI::'volt ampere'")

    @property
    def volt_ampere_reactive(self):
        return self._model.element("SI::'volt ampere reactive'")

    @property
    def volt_per_kelvin(self):
        return self._model.element("SI::'volt per kelvin'")

    @property
    def volt_per_metre(self):
        return self._model.element("SI::'volt per metre'")

    @property
    def volt_to_the_power_2_per_kelvin_to_the_power_2(self):
        return self._model.element("SI::'volt to the power 2 per kelvin to the power 2'")

    @property
    def watt(self):
        return self._model.element("SI::watt")

    @property
    def watt_hour(self):
        return self._model.element("SI::'watt hour'")

    @property
    def watt_per_kelvin(self):
        return self._model.element("SI::'watt per kelvin'")

    @property
    def watt_per_kilogram(self):
        return self._model.element("SI::'watt per kilogram'")

    @property
    def watt_per_metre_kelvin(self):
        return self._model.element("SI::'watt per metre kelvin'")

    @property
    def watt_per_nm(self):
        return self._model.element("SI::'watt per nm'")

    @property
    def watt_per_square_metre(self):
        return self._model.element("SI::'watt per square metre'")

    @property
    def watt_per_square_metre_kelvin(self):
        return self._model.element("SI::'watt per square metre kelvin'")

    @property
    def watt_per_square_metre_nm(self):
        return self._model.element("SI::'watt per square metre nm'")

    @property
    def watt_per_steradian(self):
        return self._model.element("SI::'watt per steradian'")

    @property
    def watt_per_steradian_nm(self):
        return self._model.element("SI::'watt per steradian nm'")

    @property
    def watt_per_steradian_square_metre(self):
        return self._model.element("SI::'watt per steradian square metre'")

    @property
    def watt_per_steradian_square_metre_nm(self):
        return self._model.element("SI::'watt per steradian square metre nm'")

    @property
    def weber(self):
        return self._model.element("SI::weber")

    @property
    def weber_metre(self):
        return self._model.element("SI::'weber metre'")

    @property
    def weber_per_metre(self):
        return self._model.element("SI::'weber per metre'")

    @property
    def zeroDegreeCelsiusInKelvin(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::zeroDegreeCelsiusInKelvin")

    @property
    def zeroDegreeCelsiusToKelvinShift(self):
        return self._model.element("SI::'degree celsius (absolute temperature scale)'::zeroDegreeCelsiusToKelvinShift")

