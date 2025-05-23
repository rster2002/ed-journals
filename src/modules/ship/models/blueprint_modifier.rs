use serde::{Deserialize, Serialize};

/// When applying blueprint to modules, modifiers are applied to the modules which are the things
/// that actually change the stats of the module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BlueprintModifier {
    Size,
    Class,
    Mass,
    Integrity,
    PowerDraw,
    BootTime,
    ShieldBankSpinUp,
    ShieldBankDuration,
    ShieldBankReinforcement,
    ShieldBankHeat,
    DamagePerSecond,
    Damage,
    DistributorDraw,
    ThermalLoad,
    ArmourPenetration,
    MaximumRange,
    ShotSpeed,
    RateOfFire,
    BurstRateOfFire,
    BurstSize,
    AmmoClipSize,
    AmmoMaximum,
    RoundsPerShot,
    ReloadTime,
    BreachDamage,
    MinBreachChance,
    MaxBreachChance,
    Jitter,
    WeaponMode,
    DamageType,
    ShieldGenMinimumMass,
    ShieldGenOptimalMass,
    ShieldGenMaximumMass,
    ShieldGenMinStrength,
    ShieldGenStrength,
    ShieldGenMaxStrength,
    RegenRate,
    BrokenRegenRate,
    EnergyPerRegen,
    FSDOptimalMass,
    FSDHeatRate,
    MaxFuelPerJump,
    EngineMinimumMass,
    EngineOptimalMass,
    MaximumMass,
    EngineMinPerformance,
    EngineOptPerformance,
    EngineMaxPerformance,
    EngineHeatRate,
    PowerCapacity,
    HeatEfficiency,
    WeaponsCapacity,
    WeaponsRecharge,
    EnginesCapacity,
    EnginesRecharge,
    SystemsCapacity,
    SystemsRecharge,
    DefenceModifierHealthMultiplier,
    DefenceModifierHealthAddition,
    DefenceModifierShieldMultiplier,
    DefenceModifierShieldAddition,
    KineticResistance,
    ThermicResistance,
    ExplosiveResistance,
    CausticResistance,
    FSDInterdictorRange,
    FSDInterdictorFacingLimit,
    ScannerRange,
    DiscoveryScannerRange,
    DiscoveryScannerPassiveRange,
    MaxAngle,
    ScannerTimeToScan,
    ChaffJamDuration,
    ECMRange,
    ECMTimeToCharge,
    ECMActivePowerConsumption,
    ECMHeat,
    ECMCooldown,
    HeatSinkDuration,
    ThermalDrain,
    NumBuggySlots,
    CargoCapacity,
    MaxActiveDrones,
    DroneTargetRange,
    DroneLifeTime,
    DroneSpeed,
    DroneMultiTargetSpeed,
    DroneFuelCapacity,
    DroneRepairCapacity,
    DroneHackingTime,
    DroneMinJettisonedCargo,
    DroneMaxJettisonedCargo,
    FuelScoopRate,
    FuelCapacity,
    OxygenTimeCapacity,
    RefineryBins,
    AFMRepairCapacity,
    AFMRepairConsumption,
    AFMRepairPerAmmo,
    MaxRange,
    SensorTargetScanAngle,
    Range,
    VehicleCargoCapacity,
    VehicleHullMass,
    VehicleFuelCapacity,
    VehicleArmourHealth,
    VehicleShieldHealth,
    FighterMaxSpeed,
    FighterBoostSpeed,
    FighterPitchRate,
    FighterDPS,
    FighterYawRate,
    FighterRollRate,
    CabinCapacity,
    CabinClass,
    DisruptionBarrierRange,
    DisruptionBarrierChargeDuration,
    DisruptionBarrierActivePower,
    DisruptionBarrierCooldown,
    WingDamageReduction,
    WingMinDuration,
    WingMaxDuration,
    ShieldSacrificeAmountRemoved,
    ShieldSacrificeAmountGiven,
    FSDJumpRangeBoost,
    FSDFuelUseIncrease,
    BoostSpeedMultiplier,
    BoostAugmenterPowerUse,
    ModuleDefenceAbsorption,
    FalloffRange,

    #[serde(rename = "DSS_RangeMult")]
    DSSRangeMultiplier,

    #[serde(rename = "DSS_AngleMult")]
    DSSAngleMultiplier,

    #[serde(rename = "DSS_RateMult")]
    DSSRateMultiplier,

    #[serde(rename = "DSS_PatchRadius")]
    DSSPatchRadius,

    DamageFalloffRange,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}
