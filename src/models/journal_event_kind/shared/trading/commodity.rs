use serde::Deserialize;

/// Includes both entries for both ship commodities and Odyssey commodities.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Commodity {
    // Chemicals
    #[serde(alias = "argonomictreatment")]
    ArgonomicTreatment,

    #[serde(alias = "explosives")]
    Explosives,

    #[serde(alias = "hydrogenfuel")]
    HydrogenFuel,

    #[serde(alias = "hydrogenperoxide")]
    HydrogenPeroxide,

    #[serde(alias = "liquidoxygen")]
    LiquidOxygen,

    #[serde(alias = "mineraloil")]
    MineralOil,

    #[serde(alias = "nerveagents")]
    NerveAgents,

    #[serde(alias = "pesticides")]
    Pesticides,

    #[serde(alias = "rockforthfertiliser")]
    RockforthFertiliser,

    #[serde(alias = "surfacestabilisers")]
    SurfaceStabilisers,

    #[serde(alias = "syntheticreagents")]
    SyntheticReagents,

    #[serde(alias = "tritium")]
    Tritium,

    #[serde(alias = "water")]
    Water,

    // Consumer items
    #[serde(alias = "clothing")]
    Clothing,

    #[serde(alias = "consumertechnology")]
    ConsumerTechnology,

    #[serde(alias = "domesticappliances")]
    DomesticAppliances,

    #[serde(alias = "evacuationshelter")]
    EvacuationShelter,

    #[serde(alias = "survivalequipment")]
    SurvivalEquipment,

    // Legal drugs
    #[serde(alias = "beer")]
    Beer,

    #[serde(alias = "bootlegliquor")]
    BootlegLiquor,

    #[serde(alias = "liquor")]
    Liquor,

    #[serde(alias = "basicnarcotics")]
    Narcotics,

    #[serde(alias = "onionheadgammastrain")]
    OnionheadGammaStrain,

    #[serde(alias = "tobacco")]
    Tobacco,

    #[serde(alias = "wine")]
    Wine,

    // Foods
    #[serde(alias = "algae")]
    Algae,

    #[serde(alias = "animalmeat")]
    AnimalMeat,

    #[serde(alias = "coffee")]
    Coffee,

    #[serde(alias = "fish")]
    Fish,

    #[serde(alias = "foodcartridges")]
    FoodCartridges,

    #[serde(alias = "fruitandvegetables")]
    FruitAndVegetables,

    #[serde(alias = "grain")]
    Grain,

    #[serde(alias = "syntheticmeat")]
    SyntheticMeat,

    #[serde(alias = "tea")]
    Tea,

    // Industrial materials
    #[serde(alias = "ceramiccomposites")]
    CeramicComposites,

    #[serde(alias = "cmmcomposite")]
    CMMComposite,

    #[serde(alias = "insulatingmembrane")]
    InsulatingMembrane,

    #[serde(alias = "metaalloys")]
    MetaAlloys,

    #[serde(alias = "microweavecollinghoses")]
    MicroWeaveCollingHoses,

    #[serde(alias = "neofabricinsulation")]
    NeofabricInsulation,

    #[serde(alias = "polymers")]
    Polymers,

    #[serde(alias = "semiconductors")]
    Semiconductors,

    #[serde(alias = "superconductors")]
    Superconductors,

    // Machinery
    #[serde(alias = "articulationmotors")]
    ArticulationMotors,

    #[serde(alias = "atmosphericprocessors")]
    AtmosphericProcessors,

    #[serde(alias = "buildingfabricators")]
    BuildingFabricators,

    #[serde(alias = "cropharvesters")]
    CropHarvesters,

    #[serde(alias = "emergencypowercells")]
    EmergencyPowerCells,

    #[serde(alias = "energygridassembly")]
    EnergyGridAssembly,

    #[serde(alias = "exhaustmanifold")]
    ExhaustManifold,

    #[serde(alias = "geologicalequipment")]
    GeologicalEquipment,

    #[serde(alias = "heatsinkinterlink")]
    HeatsinkInterlink,

    #[serde(alias = "hnshockmount")]
    HNShockMount,

    #[serde(alias = "iondistributor")]
    IonDistributor,

    #[serde(alias = "magneticemittercoil")]
    MagneticEmitterCoil,

    #[serde(alias = "marineequipment")]
    MarineEquipment,

    #[serde(alias = "microbialfurnaces")]
    MicrobialFurnaces,

    #[serde(alias = "mineralextractors")]
    MineralExtractors,

    #[serde(alias = "modularterminals")]
    ModularTerminals,

    #[serde(alias = "powerconverter")]
    PowerConverter,

    #[serde(alias = "powergenerators")]
    PowerGenerators,

    #[serde(alias = "powertransferbus")]
    PowerTransferBus,

    #[serde(alias = "radiationbaffle")]
    RadiationBaffle,

    #[serde(alias = "reinforcedmountingplate")]
    ReinforcedMountingPlate,

    #[serde(alias = "skimmercomponents")]
    SkimmerComponents,

    #[serde(alias = "thermalcoolingunits")]
    ThermalCoolingUnits,

    #[serde(alias = "waterpurifiers")]
    WaterPurifiers,

    // Medicines
    #[serde(alias = "advancedmedicines")]
    AdvancedMedicines,

    #[serde(alias = "agrimedicines")]
    AgriMedicines,

    #[serde(alias = "basicmedicines")]
    BasicMedicines,

    #[serde(alias = "combatstabilisers")]
    CombatStabilisers,

    #[serde(alias = "performanceenhancers")]
    PerformanceEnhancers,

    #[serde(alias = "progenitorcells")]
    ProgenitorCells,

    // Metals
    #[serde(alias = "aluminium")]
    Aluminium,

    #[serde(alias = "beryllium")]
    Beryllium,

    #[serde(alias = "bismuth")]
    Bismuth,

    #[serde(alias = "cobalt")]
    Cobalt,

    #[serde(alias = "copper")]
    Copper,

    #[serde(alias = "gallium")]
    Gallium,

    #[serde(alias = "gold")]
    Gold,

    #[serde(alias = "hafnium178")]
    Hafnium178,

    #[serde(alias = "indium")]
    Indium,

    #[serde(alias = "lanthanum")]
    Lanthanum,

    #[serde(alias = "lithium")]
    Lithium,

    #[serde(alias = "osmium")]
    Osmium,

    #[serde(alias = "palladium")]
    Palladium,

    #[serde(alias = "platinum")]
    Platinum,

    #[serde(alias = "praseodymium")]
    Praseodymium,

    #[serde(alias = "samarium")]
    Samarium,

    #[serde(alias = "silver", alias = "$Silver_Name;")]
    Silver,

    #[serde(alias = "tantalum")]
    Tantalum,

    #[serde(alias = "thallium")]
    Thallium,

    #[serde(alias = "thorium")]
    Thorium,

    #[serde(alias = "titanium")]
    Titanium,

    #[serde(alias = "uranium")]
    Uranium,

    // Minerals
    #[serde(alias = "alexandrite")]
    Alexandrite,

    #[serde(alias = "bauxite")]
    Bauxite,

    #[serde(alias = "benitoite")]
    Benitoite,

    #[serde(alias = "bertrandite")]
    Bertrandite,

    #[serde(alias = "bromellite")]
    Bromellite,

    #[serde(alias = "coltan")]
    Coltan,

    #[serde(alias = "cryolite")]
    Cryolite,

    #[serde(alias = "gallite")]
    Gallite,

    #[serde(alias = "goslarite")]
    Goslarite,

    #[serde(alias = "grandidierite")]
    Grandidierite,

    #[serde(alias = "indite", alias = "$Indite_Name;")]
    Indite,

    #[serde(alias = "jadeite")]
    Jadeite,

    #[serde(alias = "lepidolite")]
    Lepidolite,

    #[serde(alias = "lithiumhydroxide")]
    LithiumHydroxide,

    #[serde(alias = "lowtemperaturediamonds")]
    LowTemperatureDiamonds,

    #[serde(alias = "methaneclathrate")]
    MethaneClathrate,

    #[serde(alias = "methanolmonohydratecrystals")]
    MethanolMonohydrateCrystals,

    #[serde(alias = "moissanite")]
    Moissanite,

    #[serde(alias = "monazite")]
    Monazite,

    #[serde(alias = "musgravite")]
    Musgravite,

    #[serde(alias = "painite")]
    Painite,

    #[serde(alias = "pyrophyllite")]
    Pyrophyllite,

    #[serde(alias = "rhodplumsite")]
    Rhodplumsite,

    #[serde(alias = "rutile")]
    Rutile,

    #[serde(alias = "serendibite")]
    Serendibite,

    #[serde(alias = "taaffeite")]
    Taaffeite,

    #[serde(alias = "uraninite")]
    Uraninite,

    #[serde(alias = "voidopal")]
    VoidOpal,

    // Salvage
    #[serde(alias = "airelics")]
    AIRelics,

    #[serde(alias = "ancientartefact")]
    AncientArtefact,

    #[serde(alias = "ancientkey")]
    AncientKey,

    #[serde(alias = "anomalyparticles")]
    AnomalyParticles,

    #[serde(alias = "antimattercontainmentunit")]
    AntimatterContainmentUnit,

    #[serde(alias = "aniquejewellery")]
    AniqueJewellery,

    #[serde(alias = "antiquities")]
    Antiquities,

    #[serde(alias = "assaultplans")]
    AssaultPlans,

    #[serde(alias = "blackbox")]
    BlackBox,

    #[serde(alias = "bonefragments")]
    BoneFragments,

    #[serde(alias = "caustictissuesample")]
    CausticTissueSample,

    #[serde(alias = "commercialsamples")]
    CommercialSamples,

    #[serde(alias = "coralsap")]
    CoralSap,

    #[serde(alias = "cystspecimen")]
    CystSpecimen,

    #[serde(alias = "damagedescapepod")]
    DamagedEscapePod,

    #[serde(alias = "datacore")]
    DataCore,

    #[serde(alias = "diplomaticbag")]
    DiplomaticBag,

    #[serde(alias = "earthrelics")]
    EarthRelics,

    #[serde(alias = "encryptedcorrespondence")]
    EncryptedCorrespondence,

    #[serde(alias = "encrypteddatastorage")]
    EncryptedDataStorage,

    #[serde(alias = "experimentalchemicals")]
    ExperimentalChemicals,

    #[serde(alias = "fossilremnants")]
    FossilRemnants,

    #[serde(alias = "genebank")]
    GeneBank,

    #[serde(alias = "geologicalsamples")]
    GeologicalSamples,

    #[serde(alias = "guardiancasket")]
    GuardianCasket,

    #[serde(alias = "guardianorb")]
    GuardianOrb,

    #[serde(alias = "AncientRelic", alias = "ancientrelic")]
    GuardianRelic,

    #[serde(alias = "guardiantablet")]
    GuardianTablet,

    #[serde(alias = "guardiantotem")]
    GuardianTotem,

    #[serde(alias = "guardianurn")]
    GuardianUrn,

    #[serde(alias = "hostages")]
    Hostages,

    #[serde(alias = "impurespiremineral")]
    ImpureSpireMineral,

    #[serde(alias = "largesurveydatacache")]
    LargeSurveyDataCache,

    #[serde(alias = "militaryintelligence")]
    MilitaryIntelligence,

    #[serde(alias = "militaryplans")]
    MilitaryPlans,

    #[serde(alias = "molluscbraintissue")]
    MolluscBrainTissue,

    #[serde(alias = "molluscfluid")]
    MolluscFluid,

    #[serde(alias = "molluscmembrane")]
    MolluscMembrane,

    #[serde(alias = "molluscmycelium")]
    MolluscMycelium,

    #[serde(alias = "molluscsofttissue")]
    MolluscSoftTissue,

    #[serde(alias = "molluscspores")]
    MolluscSpores,

    #[serde(alias = "mysteriousidol")]
    MysteriousIdol,

    #[serde(alias = "occupiedescapepod")]
    OccupiedEscapePod,

    #[serde(alias = "organsample")]
    OrganSample,

    #[serde(alias = "personaleffects")]
    PersonalEffects,

    #[serde(alias = "podcoretissue")]
    PodCoreTissue,

    #[serde(alias = "poddeadtissue")]
    PodDeadTissue,

    #[serde(alias = "podmesoglea")]
    PodMesoglea,

    #[serde(alias = "podoutertissue")]
    PodOuterTissue,

    #[serde(alias = "podshelltissue")]
    PodShellTissue,

    #[serde(alias = "podsurfacetissue")]
    PodSurfaceTissue,

    #[serde(alias = "podtissue")]
    PodTissue,

    #[serde(alias = "politicalprisoners")]
    PoliticalPrisoners,

    #[serde(alias = "preciousgems")]
    PreciousGems,

    #[serde(alias = "prohibitedresearchmaterials")]
    ProhibitedResearchMaterials,

    #[serde(alias = "protectivemembranescrap")]
    ProtectiveMembraneScrap,

    #[serde(alias = "prototypetech")]
    PrototypeTech,

    #[serde(alias = "rareartwork")]
    RareArtwork,

    #[serde(alias = "rebeltransmission")]
    RebelTransmission,

    #[serde(alias = "sap8corecontainer")]
    SAP8CoreContainer,

    #[serde(alias = "scientificresearch")]
    ScientificResearch,

    #[serde(alias = "scientificsamples")]
    ScientificSamples,

    #[serde(alias = "semirefinesspiremineral")]
    SemiRefinesSpireMineral,

    #[serde(alias = "smallsurveydatacache")]
    SmallSurveyDataCache,

    #[serde(alias = "spacepioneerrelics")]
    SpacePioneerRelics,

    #[serde(alias = "tacticaldata")]
    TacticalData,

    #[serde(alias = "technicalblueprints")]
    TechnicalBlueprints,

    #[serde(alias = "thargoidbasilisktissuesample")]
    ThargoidBasiliskTissueSample,

    #[serde(alias = "thargoidbiostoragecapsule")]
    ThargoidBioStorageCapsule,

    #[serde(alias = "thargoidbiologicalmatter")]
    ThargoidBiologicalMatter,

    #[serde(alias = "thargoidcyclopstissuesample")]
    ThargoidCyclopsTissueSample,

    #[serde(alias = "thargoidglaivetissuesample")]
    ThargoidGlaiveTissueSample,

    #[serde(alias = "thargoidheart")]
    ThargoidHeart,

    #[serde(alias = "thargoidhydratissuesample")]
    ThargoidHydraTissueSample,

    #[serde(alias = "thargoidlink")]
    ThargoidLink,

    #[serde(alias = "thargoidmedusatissuesample")]
    ThargoidMedusaTissueSample,

    #[serde(alias = "thargoidorthrustissuesample")]
    ThargoidOrthrusTissueSample,

    #[serde(alias = "thargoidprobe")]
    ThargoidProbe,

    #[serde(alias = "thargoidresin")]
    ThargoidResin,

    #[serde(alias = "thargoidscouttissuesample")]
    ThargoidScoutTissueSample,

    #[serde(alias = "thargoidscythetissuesample")]
    ThargoidScytheTissueSample,

    #[serde(alias = "ThargoidGeneratorTissueSample", alias = "thargoidgeneratortissuesample")]
    ThargoidGeneratorTissueSample,

    #[serde(alias = "thargoidsensor")]
    ThargoidSensor,

    #[serde(alias = "thargoidtechnologysamples")]
    ThargoidTechnologySamples,

    #[serde(alias = "timecapsule")]
    TimeCapsule,

    #[serde(alias = "titandeeptissuesample")]
    TitanDeepTissueSample,

    #[serde(alias = "titandrivecomponent")]
    TitanDriveComponent,

    #[serde(alias = "titanmawdeeptissuesample")]
    TitanMawDeepTissueSample,

    #[serde(alias = "titanmawpartialtissuesample")]
    TitanMawPartialTissueSample,

    #[serde(alias = "titanmawtissuesample")]
    TitanMawTissueSample,

    #[serde(alias = "titanpartialtissuesample")]
    TitanPartialTissueSample,

    #[serde(alias = "titantissuesample")]
    TitanTissueSample,

    #[serde(alias = "tradedata")]
    TradeData,

    #[serde(alias = "trinketsofhiddenfortune")]
    TrinketsOfHiddenFortune,

    #[serde(alias = "unclassifiedrelic")]
    UnclassifiedRelic,

    #[serde(alias = "unoccupiedescapepod")]
    UnoccupiedEscapePod,

    #[serde(alias = "unstabledatacore")]
    UnstableDataCore,

    #[serde(alias = "wreckagecomponents")]
    WreckageComponents,

    // Slaves
    #[serde(alias = "imperialslaves")]
    ImperialSlaves,

    #[serde(alias = "slaves")]
    Slaves,

    // Technology
    #[serde(alias = "advancedcatalysers")]
    AdvancedCatalysers,

    #[serde(alias = "animalmonitors")]
    AnimalMonitors,

    #[serde(alias = "aquaponicsystems")]
    AquaponicSystems,

    #[serde(alias = "autofabricators")]
    AutoFabricators,

    #[serde(alias = "bioreducinglichen")]
    BioreducingLichen,

    #[serde(alias = "computercomponents")]
    ComputerComponents,

    #[serde(alias = "hesuits")]
    HESuits,

    #[serde(alias = "hardwarediagnosticsensor")]
    HardwareDiagnosticSensor,

    #[serde(alias = "landenrichmentsystems")]
    LandEnrichmentSystems,

    #[serde(alias = "medialdiagnosticequipment")]
    MedialDiagnosticEquipment,

    #[serde(alias = "microcontrollers")]
    MicroControllers,

    #[serde(alias = "muonimager")]
    MuonImager,

    #[serde(alias = "nanobreakers")]
    Nanobreakers,

    #[serde(alias = "resonatingseparators")]
    ResonatingSeparators,

    #[serde(alias = "robotics")]
    Robotics,

    #[serde(alias = "structuralregulators")]
    StructuralRegulators,

    #[serde(alias = "telemetrysuite")]
    TelemetrySuite,

    // Textiles
    #[serde(alias = "conductivefabrics")]
    ConductiveFabrics,

    #[serde(alias = "leather")]
    Leather,

    #[serde(alias = "militarygradefabrics")]
    MilitaryGradeFabrics,

    #[serde(alias = "naturalfabrics")]
    NaturalFabrics,

    #[serde(alias = "syntheticfabrics")]
    SyntheticFabrics,

    // Waste
    #[serde(alias = "biowaste")]
    Biowaste,

    #[serde(alias = "chemicalwaste")]
    ChemicalWaste,

    #[serde(alias = "scrap")]
    Scrap,

    #[serde(alias = "toxicwaste")]
    ToxicWaste,

    // Weapons
    #[serde(alias = "battleweapons")]
    BattleWeapons,

    #[serde(alias = "landmines")]
    Landmines,

    #[serde(alias = "nonlethalweapons")]
    NonLethalWeapons,

    #[serde(alias = "personalweapons")]
    PersonalWeapons,

    #[serde(alias = "reactivearmour")]
    ReactiveArmour,

    // Rare
    #[serde(alias = "jaquesquinentianstill")]
    JaquesQuinentianStill,

    #[serde(alias = "kinagoviolins")]
    KinagoViolins,

    #[serde(alias = "apavietii")]
    ApaVietii,

    #[serde(alias = "geawendancedust")]
    GeawenDanceDust,

    #[serde(alias = "vanayequiceratomorphafur")]
    VanayequiCeratomorphaFur,

    #[serde(alias = "karetiicouture")]
    KaretiiCouture,

    #[serde(alias = "mukusubiichitinos")]
    MukusubiiChitinos,

    #[serde(alias = "ultracompactprocessorprototypes")]
    UltraCompactProcessorPrototypes,

    #[serde(alias = "eleuthermals")]
    EleuThermals,

    #[serde(alias = "kamorinhistoricweapons")]
    KamorinHistoricWeapons,

    #[serde(alias = "ceremonialheiketea")]
    CeremonialHeikeTea,

    #[serde(alias = "vidavantianlace")]
    VidavantianLace,

    #[serde(alias = "kachiriginfilterleeches")]
    KachiriginFilterLeeches,

    #[serde(alias = "lyraeweed")]
    LyraeWeed,

    #[serde(alias = "galactictravelguide")]
    GalacticTravelGuide,

    #[serde(alias = "harmasilversearum")]
    HarmaSilverSeaRum,

    #[serde(alias = "ngadandarifireopals")]
    NgadandariFireOpals,

    #[serde(alias = "alyabodysoap")]
    AlyaBodySoap,

    #[serde(alias = "helvetitjpearls")]
    HelvetitjPearls,

    #[serde(alias = "ochoengchillies")]
    OchoengChillies,

    #[serde(alias = "onionheadbetastrain")]
    OnionheadBetaStrain,

    #[serde(alias = "kamitracigars")]
    KamitraCigars,

    #[serde(alias = "njangarisaddles")]
    NjangariSaddles,

    #[serde(alias = "hiporganophosphates")]
    HipOrganophosphates,

    #[serde(alias = "gilyasignatureweapons")]
    GilyaSignatureWeapons,

    #[serde(alias = "hr7221wheat")]
    HR7221Wheat,

    #[serde(alias = "wheemetewheatcakes")]
    WheemeteWheatCakes,

    #[serde(alias = "rajukrumultistoves")]
    RajukruMultiStoves,

    #[serde(alias = "nanomedicines")]
    Nanomedicines,

    #[serde(alias = "noneuclidianexotanks")]
    NonEuclidianExotanks,

    #[serde(alias = "ngunamodernantiques")]
    NgunaModernAntiques,

    #[serde(alias = "xihebiomorphiccompanions")]
    XiheBiomorphicCompanions,

    #[serde(alias = "esusekucaviar")]
    EsusekuCaviar,

    #[serde(alias = "orrerianviciousbrew")]
    OrrerianViciousBrew,

    #[serde(alias = "vherculisbodyrub")]
    VHerculisBodyRub,

    #[serde(alias = "voidextractcoffee")]
    VoidExtractCoffee,

    #[serde(alias = "uszaiantreegrub")]
    UszaianTreeGrub,

    #[serde(alias = "haidenblackbrew")]
    HaidenBlackBrew,

    #[serde(alias = "motronaexperiencejelly")]
    MotronaExperienceJelly,

    #[serde(alias = "jaradharrepuzzlebox")]
    JaradharrePuzzleBox,

    #[serde(alias = "personalgifts")]
    PersonalGifts,

    #[serde(alias = "mulachigiantfungus")]
    MulachiGiantFungus,

    #[serde(alias = "ltthypersweet")]
    LTTHyperSweet,

    #[serde(alias = "medbstarlube")]
    MedbStarlube,

    #[serde(alias = "giantverrix")]
    GiantVerrix,

    #[serde(alias = "hip118311swarm")]
    HIP118311Swarm,

    #[serde(alias = "disomacorn")]
    DisoMaCorn,

    #[serde(alias = "lavianbrandy")]
    LavianBrandy,

    #[serde(alias = "azuremilk")]
    AzureMilk,

    #[serde(alias = "leestianeviljuice")]
    LeestianEvilJuice,

    #[serde(alias = "coquimspongiformvictuals")]
    CoquimSpongiformVictuals,

    #[serde(alias = "leatheryeggs")]
    LeatheryEggs,

    #[serde(alias = "shanscharisorchid")]
    ShansCharisOrchid,

    #[serde(alias = "konggaale")]
    KonggaAle,

    #[serde(alias = "vegaslimweed")]
    VegaSlimweed,

    #[serde(alias = "tiolcewaste2pasteunits")]
    TiolceWaste2PasteUnits,

    #[serde(alias = "ophiuchexinoartefacts")]
    OphiuchExinoArtefacts,

    #[serde(alias = "altairianskin")]
    AltairianSkin,

    #[serde(alias = "aganipperush")]
    AganippeRush,

    #[serde(alias = "cd75kittenbrandcoffee")]
    CD75KittenBrandCoffee,

    #[serde(alias = "havasupaidreamcatcher")]
    HavasupaiDreamCatcher,

    #[serde(alias = "eraninpearlwhisky")]
    EraninPearlWhisky,

    #[serde(alias = "pavoniseargrubs")]
    PavonisEarGrubs,

    #[serde(alias = "onionheadalphastrain")]
    OnionheadAlphaStrain,

    #[serde(alias = "indibourbon")]
    IndiBourbon,

    #[serde(alias = "bakedgreebles")]
    BakedGreebles,

    #[serde(alias = "karsukilocusts")]
    KarsukiLocusts,

    #[serde(alias = "masterchefs")]
    MasterChefs,

    #[serde(alias = "yasokondileaf")]
    YasoKondiLeaf,

    #[serde(alias = "burnhambiledistillate")]
    BurnhamBileDistillate,

    #[serde(alias = "thehuttonmug")]
    TheHuttonMug,

    #[serde(alias = "centaurimegagin")]
    CentauriMegaGin,

    #[serde(alias = "utgaroarmillennialeggs")]
    UtgaroarMillennialEggs,

    #[serde(alias = "soontillrelics")]
    SoontillRelics,

    #[serde(alias = "zeesszeantgrubglue")]
    ZeesszeAntGrubGlue,

    #[serde(alias = "thewatersofshintara")]
    TheWatersOfShintara,

    #[serde(alias = "baltahsinevacuumkrill")]
    BaltahsineVacuumKrill,

    #[serde(alias = "sanumadecorativemeat")]
    SanumaDecorativeMeat,

    #[serde(alias = "giantirukamasnails")]
    GiantIrukamaSnails,

    #[serde(alias = "anduligafireworks")]
    AnduligaFireWorks,

    #[serde(alias = "crystallinespheres")]
    CrystallineSpheres,

    #[serde(alias = "pantaaprayersticks")]
    PantaaPrayerSticks,

    #[serde(alias = "chieridanimarinepaste")]
    ChiEridaniMarinePaste,

    #[serde(alias = "ethgrezeteabuds")]
    EthgrezeTeaBuds,

    #[serde(alias = "deltaphoenicispalms")]
    DeltaPhoenicisPalms,

    #[serde(alias = "tarachspice")]
    TarachSpice,

    #[serde(alias = "wulpahyperboresystems")]
    WulpaHyperboreSystems,

    #[serde(alias = "livehecateseaworms")]
    LiveHecateSeaWorms,

    #[serde(alias = "korokungpellets")]
    KoroKungPellets,

    #[serde(alias = "bastsnakegin")]
    BastSnakeGin,

    #[serde(alias = "terramaterbloodbores")]
    TerraMaterBloodBores,

    #[serde(alias = "wuthielokufroth")]
    WuthieloKuFroth,

    #[serde(alias = "honestypills")]
    HonestyPills,

    #[serde(alias = "cromsilverfesh")]
    CromSilverFesh,

    #[serde(alias = "borasetanipathogenetics")]
    BorasetaniPathogenetics,

    #[serde(alias = "cetirabbits")]
    CetiRabbits,

    #[serde(alias = "aepyornisegg")]
    AepyornisEgg,

    #[serde(alias = "uzumokulowgwings")]
    UzumokuLowGWings,

    #[serde(alias = "cherbonesbloodcrystals")]
    CherbonesBloodCrystals,

    #[serde(alias = "toxandjivirocide")]
    ToxandjiVirocide,

    #[serde(alias = "onionhead")]
    Onionhead,

    #[serde(alias = "lucanonionhead")]
    LucanOnionhead,

    #[serde(alias = "tanmarktranquiltea")]
    TanmarkTranquilTea,

    #[serde(alias = "thrutiscream")]
    ThrutisCream,

    #[serde(alias = "alacarakmoskinart")]
    AlacarakmoSkinArt,

    #[serde(alias = "platinumalloy")]
    PlatinumAlloy,

    #[serde(alias = "mokojingbeastfeast")]
    MokojingBeastFeast,

    #[serde(alias = "edenapplesofaerial")]
    EdenApplesOfAerial,

    #[serde(alias = "chameleoncloth")]
    ChameleonCloth,

    #[serde(alias = "taurichimes")]
    TauriChimes,

    #[serde(alias = "rusanioldsmokey")]
    RusaniOldSmokey,

    #[serde(alias = "azcancriformula42")]
    AZCancriFormula42,

    #[serde(alias = "gomanyauponcoffee")]
    GomanYauponCoffee,

    #[serde(alias = "gerasiangueuzebeer")]
    GerasianGueuzeBeer,

    #[serde(alias = "jarouarice")]
    JarouaRice,

    #[serde(alias = "anynacoffee")]
    AnyNaCoffee,

    #[serde(alias = "fujintea")]
    FujinTea,

    #[serde(alias = "hip10175bushmeat")]
    HIP10175BushMeat,

    #[serde(alias = "momusbogspaniel")]
    MomusBogSpaniel,

    #[serde(alias = "witchhaulkobebeef")]
    WitchhaulKobeBeef,

    #[serde(alias = "saxonwine")]
    SaxonWine,

    #[serde(alias = "aroucaconventualsweets")]
    AroucaConventualSweets,

    #[serde(alias = "albinoquechuamammothmeat")]
    AlbinoQuechuaMammothMeat,

    #[serde(alias = "duradrives")]
    Duradrives,

    #[serde(alias = "holvaduellingblades")]
    HolvaDuellingBlades,

    #[serde(alias = "rapabaosnakeskins")]
    RapaBaoSnakeSkins,

    #[serde(alias = "wolffesh")]
    WolfFesh,

    #[serde(alias = "eshuumbrellas")]
    EshuUmbrellas,

    #[serde(alias = "neritusberries")]
    NeritusBerries,

    #[serde(alias = "jotunmookah")]
    JotunMookah,

    #[serde(alias = "chateaudeaegaeon")]
    ChateauDeAegaeon,

    #[serde(alias = "belalansrayleather")]
    BelalansRayLeather,

    #[serde(alias = "damnacarapaces")]
    DamnaCarapaces,

    #[serde(alias = "hipprotosquid")]
    HIPProtoSquid,

    #[serde(alias = "mechucoshightea")]
    MechucosHighTea,

    #[serde(alias = "deuringastruffles")]
    DeuringasTruffles,

    #[serde(alias = "bankiamphibiousleather")]
    BankiAmphibiousLeather,

    #[serde(alias = "sothiscrystallinegold")]
    SothisCrystallineGold,

    #[serde(alias = "tiegfriessynthsilk")]
    TiegfriesSynthSilk,

    #[serde(alias = "volkhabbeedrones")]
    VolkhabBeeDrones,

    #[serde(alias = "buckyballbeermats")]
    BuckyballBeerMats,

    #[serde(alias = "classifiedexperimentalequipment")]
    ClassifiedExperimentalEquipment,


    // Data
    #[serde(alias = "$EnhancedInterrogationRecordings_Name;")]
    EnhancedInterrogationRecordings,


    // Goods
    #[serde(alias = "$IonisedGas_Name;")]
    IonisedGas,


    // None
    #[serde(alias = "drones")]
    Limpet,


    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

