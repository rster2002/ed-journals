use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Commodity {
    // Chemicals
    #[serde(rename = "argonomictreatment")]
    ArgonomicTreatment,

    #[serde(rename = "explosives")]
    Explosives,

    #[serde(rename = "hydrogenfuel")]
    HydrogenFuel,

    #[serde(rename = "hydrogenperoxide")]
    HydrogenPeroxide,

    #[serde(rename = "liquidoxygen")]
    LiquidOxygen,

    #[serde(rename = "mineraloil")]
    MineralOil,

    #[serde(rename = "nerveagents")]
    NerveAgents,

    #[serde(rename = "pesticides")]
    Pesticides,

    #[serde(rename = "rockforthfertiliser")]
    RockforthFertiliser,

    #[serde(rename = "surfacestabilisers")]
    SurfaceStabilisers,

    #[serde(rename = "syntheticreagents")]
    SyntheticReagents,

    #[serde(rename = "tritium")]
    Tritium,

    #[serde(rename = "water")]
    Water,

    // Consumer items
    #[serde(rename = "clothing")]
    Clothing,

    #[serde(rename = "consumertechnology")]
    ConsumerTechnology,

    #[serde(rename = "domesticappliances")]
    DomesticAppliances,

    #[serde(rename = "evacuationshelter")]
    EvacuationShelter,

    #[serde(rename = "survivalequipment")]
    SurvivalEquipment,

    // Legal drugs
    #[serde(rename = "beer")]
    Beer,

    #[serde(rename = "bootlegliquor")]
    BootlegLiquor,

    #[serde(rename = "liquor")]
    Liquor,

    #[serde(rename = "basicnarcotics")]
    Narcotics,

    #[serde(rename = "onionheadgammastrain")]
    OnionheadGammaStrain,

    #[serde(rename = "tobacco")]
    Tobacco,

    #[serde(rename = "wine")]
    Wine,

    // Foods
    #[serde(rename = "algae")]
    Algae,

    #[serde(rename = "animalmeat")]
    AnimalMeat,

    #[serde(rename = "coffee")]
    Coffee,

    #[serde(rename = "fish")]
    Fish,

    #[serde(rename = "foodcartridges")]
    FoodCartridges,

    #[serde(rename = "fruitandvegetables")]
    FruitAndVegetables,

    #[serde(rename = "grain")]
    Grain,

    #[serde(rename = "syntheticmeat")]
    SyntheticMeat,

    #[serde(rename = "tea")]
    Tea,

    // Industrial materials
    #[serde(rename = "ceramiccomposites")]
    CeramicComposites,

    #[serde(rename = "cmmcomposite")]
    CMMComposite,

    #[serde(rename = "insulatingmembrane")]
    InsulatingMembrane,

    #[serde(rename = "metaalloys")]
    MetaAlloys,

    #[serde(rename = "microweavecollinghoses")]
    MicroWeaveCollingHoses,

    #[serde(rename = "neofabricinsulation")]
    NeofabricInsulation,

    #[serde(rename = "polymers")]
    Polymers,

    #[serde(rename = "semiconductors")]
    Semiconductors,

    #[serde(rename = "superconductors")]
    Superconductors,

    // Machinery
    #[serde(rename = "articulationmotors")]
    ArticulationMotors,

    #[serde(rename = "atmosphericprocessors")]
    AtmosphericProcessors,

    #[serde(rename = "buildingfabricators")]
    BuildingFabricators,

    #[serde(rename = "cropharvesters")]
    CropHarvesters,

    #[serde(rename = "emergencypowercells")]
    EmergencyPowerCells,

    #[serde(rename = "energygridassembly")]
    EnergyGridAssembly,

    #[serde(rename = "exhaustmanifold")]
    ExhaustManifold,

    #[serde(rename = "geologicalequipment")]
    GeologicalEquipment,

    #[serde(rename = "heatsinkinterlink")]
    HeatsinkInterlink,

    #[serde(rename = "hnshockmount")]
    HNShockMount,

    #[serde(rename = "iondistributor")]
    IonDistributor,

    #[serde(rename = "magneticemittercoil")]
    MagneticEmitterCoil,

    #[serde(rename = "marineequipment")]
    MarineEquipment,

    #[serde(rename = "microbialfurnaces")]
    MicrobialFurnaces,

    #[serde(rename = "mineralextractors")]
    MineralExtractors,

    #[serde(rename = "modularterminals")]
    ModularTerminals,

    #[serde(rename = "powerconverter")]
    PowerConverter,

    #[serde(rename = "powergenerators")]
    PowerGenerators,

    #[serde(rename = "powertransferbus")]
    PowerTransferBus,

    #[serde(rename = "radiationbaffle")]
    RadiationBaffle,

    #[serde(rename = "reinforcedmountingplate")]
    ReinforcedMountingPlate,

    #[serde(rename = "skimmercomponents")]
    SkimmerComponents,

    #[serde(rename = "thermalcoolingunits")]
    ThermalCoolingUnits,

    #[serde(rename = "waterpurifiers")]
    WaterPurifiers,

    // Medicines
    #[serde(rename = "advancedmedicines")]
    AdvancedMedicines,

    #[serde(rename = "agrimedicines")]
    AgriMedicines,

    #[serde(rename = "basicmedicines")]
    BasicMedicines,

    #[serde(rename = "combatstabilisers")]
    CombatStabilisers,

    #[serde(rename = "performanceenhancers")]
    PerformanceEnhancers,

    #[serde(rename = "progenitorcells")]
    ProgenitorCells,

    // Metals
    #[serde(rename = "aluminium")]
    Aluminium,

    #[serde(rename = "beryllium")]
    Beryllium,

    #[serde(rename = "bismuth")]
    Bismuth,

    #[serde(rename = "cobalt")]
    Cobalt,

    #[serde(rename = "copper")]
    Copper,

    #[serde(rename = "gallium")]
    Gallium,

    #[serde(rename = "gold")]
    Gold,

    #[serde(rename = "hafnium178")]
    Hafnium178,

    #[serde(rename = "indium")]
    Indium,

    #[serde(rename = "lanthanum")]
    Lanthanum,

    #[serde(rename = "lithium")]
    Lithium,

    #[serde(rename = "osmium")]
    Osmium,

    #[serde(rename = "palladium")]
    Palladium,

    #[serde(rename = "platinum")]
    Platinum,

    #[serde(rename = "praseodymium")]
    Praseodymium,

    #[serde(rename = "samarium")]
    Samarium,

    #[serde(rename = "silver")]
    Silver,

    #[serde(rename = "tantalum")]
    Tantalum,

    #[serde(rename = "thallium")]
    Thallium,

    #[serde(rename = "thorium")]
    Thorium,

    #[serde(rename = "titanium")]
    Titanium,

    #[serde(rename = "uranium")]
    Uranium,

    // Minerals
    #[serde(rename = "alexandrite")]
    Alexandrite,

    #[serde(rename = "bauxite")]
    Bauxite,

    #[serde(rename = "benitoite")]
    Benitoite,

    #[serde(rename = "bertrandite")]
    Bertrandite,

    #[serde(rename = "bromellite")]
    Bromellite,

    #[serde(rename = "coltan")]
    Coltan,

    #[serde(rename = "cryolite")]
    Cryolite,

    #[serde(rename = "gallite")]
    Gallite,

    #[serde(rename = "goslarite")]
    Goslarite,

    #[serde(rename = "grandidierite")]
    Grandidierite,

    #[serde(rename = "indite")]
    Indite,

    #[serde(rename = "jadeite")]
    Jadeite,

    #[serde(rename = "lepidolite")]
    Lepidolite,

    #[serde(rename = "lithiumhydroxide")]
    LithiumHydroxide,

    #[serde(rename = "lowtemperaturediamonds")]
    LowTemperatureDiamonds,

    #[serde(rename = "methaneclathrate")]
    MethaneClathrate,

    #[serde(rename = "methanolmonohydratecrystals")]
    MethanolMonohydrateCrystals,

    #[serde(rename = "moissanite")]
    Moissanite,

    #[serde(rename = "monazite")]
    Monazite,

    #[serde(rename = "musgravite")]
    Musgravite,

    #[serde(rename = "painite")]
    Painite,

    #[serde(rename = "pyrophyllite")]
    Pyrophyllite,

    #[serde(rename = "rhodplumsite")]
    Rhodplumsite,

    #[serde(rename = "rutile")]
    Rutile,

    #[serde(rename = "serendibite")]
    Serendibite,

    #[serde(rename = "taaffeite")]
    Taaffeite,

    #[serde(rename = "uraninite")]
    Uraninite,

    #[serde(rename = "voidopal")]
    VoidOpal,

    // Salvage
    #[serde(rename = "airelics")]
    AIRelics,

    #[serde(rename = "ancientartefact")]
    AncientArtefact,

    #[serde(rename = "ancientkey")]
    AncientKey,

    #[serde(rename = "anomalyparticles")]
    AnomalyParticles,

    #[serde(rename = "antimattercontainmentunit")]
    AntimatterContainmentUnit,

    #[serde(rename = "aniquejewellery")]
    AniqueJewellery,

    #[serde(rename = "antiquities")]
    Antiquities,

    #[serde(rename = "assaultplans")]
    AssaultPlans,

    #[serde(rename = "blackbox")]
    BlackBox,

    #[serde(rename = "bonefragments")]
    BoneFragments,

    #[serde(rename = "caustictissuesample")]
    CausticTissueSample,

    #[serde(rename = "commercialsamples")]
    CommercialSamples,

    #[serde(rename = "coralsap")]
    CoralSap,

    #[serde(rename = "cystspecimen")]
    CystSpecimen,

    #[serde(rename = "damagedescapepod")]
    DamagedEscapePod,

    #[serde(rename = "datacore")]
    DataCore,

    #[serde(rename = "diplomaticbag")]
    DiplomaticBag,

    #[serde(rename = "earthrelics")]
    EarthRelics,

    #[serde(rename = "encryptedcorrespondence")]
    EncryptedCorrespondence,

    #[serde(rename = "encrypteddatastorage")]
    EncryptedDataStorage,

    #[serde(rename = "experimentalchemicals")]
    ExperimentalChemicals,

    #[serde(rename = "fossilremnants")]
    FossilRemnants,

    #[serde(rename = "genebank")]
    GeneBank,

    #[serde(rename = "geologicalsamples")]
    GeologicalSamples,

    #[serde(rename = "guardiancasket")]
    GuardianCasket,

    #[serde(rename = "guardianorb")]
    GuardianOrb,

    #[serde(rename = "guardianrelic")]
    GuardianRelic,

    #[serde(rename = "guardiantablet")]
    GuardianTablet,

    #[serde(rename = "guardiantotem")]
    GuardianTotem,

    #[serde(rename = "guardianurn")]
    GuardianUrn,

    #[serde(rename = "hostages")]
    Hostages,

    #[serde(rename = "impurespiremineral")]
    ImpureSpireMineral,

    #[serde(rename = "largesurveydatacache")]
    LargeSurveyDataCache,

    #[serde(rename = "militaryintelligence")]
    MilitaryIntelligence,

    #[serde(rename = "militaryplans")]
    MilitaryPlans,

    #[serde(rename = "molluscbraintissue")]
    MolluscBrainTissue,

    #[serde(rename = "molluscfluid")]
    MolluscFluid,

    #[serde(rename = "molluscmembrane")]
    MolluscMembrane,

    #[serde(rename = "molluscmycelium")]
    MolluscMycelium,

    #[serde(rename = "molluscsofttissue")]
    MolluscSoftTissue,

    #[serde(rename = "molluscspores")]
    MolluscSpores,

    #[serde(rename = "mysteriousidol")]
    MysteriousIdol,

    #[serde(rename = "occupiedescapepod")]
    OccupiedEscapePod,

    #[serde(rename = "organsample")]
    OrganSample,

    #[serde(rename = "personaleffects")]
    PersonalEffects,

    #[serde(rename = "podcoretissue")]
    PodCoreTissue,

    #[serde(rename = "poddeadtissue")]
    PodDeadTissue,

    #[serde(rename = "podmesoglea")]
    PodMesoglea,

    #[serde(rename = "podoutertissue")]
    PodOuterTissue,

    #[serde(rename = "podshelltissue")]
    PodShellTissue,

    #[serde(rename = "podsurfacetissue")]
    PodSurfaceTissue,

    #[serde(rename = "podtissue")]
    PodTissue,

    #[serde(rename = "politicalprisoners")]
    PoliticalPrisoners,

    #[serde(rename = "preciousgems")]
    PreciousGems,

    #[serde(rename = "prohibitedresearchmaterials")]
    ProhibitedResearchMaterials,

    #[serde(rename = "protectivemembranescrap")]
    ProtectiveMembraneScrap,

    #[serde(rename = "prototypetech")]
    PrototypeTech,

    #[serde(rename = "rareartwork")]
    RareArtwork,

    #[serde(rename = "rebeltransmission")]
    RebelTransmission,

    #[serde(rename = "sap8corecontainer")]
    SAP8CoreContainer,

    #[serde(rename = "scientificresearch")]
    ScientificResearch,

    #[serde(rename = "scientificsamples")]
    ScientificSamples,

    #[serde(rename = "semirefinesspiremineral")]
    SemiRefinesSpireMineral,

    #[serde(rename = "smallsurveydatacache")]
    SmallSurveyDataCache,

    #[serde(rename = "spacepioneerrelics")]
    SpacePioneerRelics,

    #[serde(rename = "tacticaldata")]
    TacticalData,

    #[serde(rename = "technicalblueprints")]
    TechnicalBlueprints,

    #[serde(rename = "thargoidbasilisktissuesample")]
    ThargoidBasiliskTissueSample,

    #[serde(rename = "thargoidbiostoragecapsule")]
    ThargoidBioStorageCapsule,

    #[serde(rename = "thargoidbiologicalmatter")]
    ThargoidBiologicalMatter,

    #[serde(rename = "thargoidcyclopstissuesample")]
    ThargoidCyclopsTissueSample,

    #[serde(rename = "thargoidglaivetissuesample")]
    ThargoidGlaiveTissueSample,

    #[serde(rename = "thargoidheart")]
    ThargoidHeart,

    #[serde(rename = "thargoidhydratissuesample")]
    ThargoidHydraTissueSample,

    #[serde(rename = "thargoidlink")]
    ThargoidLink,

    #[serde(rename = "thargoidmedusatissuesample")]
    ThargoidMedusaTissueSample,

    #[serde(rename = "thargoidorthrustissuesample")]
    ThargoidOrthrusTissueSample,

    #[serde(rename = "thargoidprobe")]
    ThargoidProbe,

    #[serde(rename = "thargoidresin")]
    ThargoidResin,

    #[serde(rename = "thargoidscouttissuesample")]
    ThargoidScoutTissueSample,

    #[serde(rename = "thargoidscythetissuesample")]
    ThargoidScytheTissueSample,

    #[serde(rename = "thargoidsensor")]
    ThargoidSensor,

    #[serde(rename = "thargoidtechnologysamples")]
    ThargoidTechnologySamples,

    #[serde(rename = "timecapsule")]
    TimeCapsule,

    #[serde(rename = "titandeeptissuesample")]
    TitanDeepTissueSample,

    #[serde(rename = "titandrivecomponent")]
    TitanDriveComponent,

    #[serde(rename = "titanmawdeeptissuesample")]
    TitanMawDeepTissueSample,

    #[serde(rename = "titanmawpartialtissuesample")]
    TitanMawPartialTissueSample,

    #[serde(rename = "titanmawtissuesample")]
    TitanMawTissueSample,

    #[serde(rename = "titanpartialtissuesample")]
    TitanPartialTissueSample,

    #[serde(rename = "titantissuesample")]
    TitanTissueSample,

    #[serde(rename = "tradedata")]
    TradeData,

    #[serde(rename = "trinketsofhiddenfortune")]
    TrinketsOfHiddenFortune,

    #[serde(rename = "unclassifiedrelic")]
    UnclassifiedRelic,

    #[serde(rename = "unoccupiedescapepod")]
    UnoccupiedEscapePod,

    #[serde(rename = "unstabledatacore")]
    UnstableDataCore,

    #[serde(rename = "wreckagecomponents")]
    WreckageComponents,

    // Slaves
    #[serde(rename = "imperialslaves")]
    ImperialSlaves,

    #[serde(rename = "slaves")]
    Slaves,

    // Technology
    #[serde(rename = "advancedcatalysers")]
    AdvancedCatalysers,

    #[serde(rename = "animalmonitors")]
    AnimalMonitors,

    #[serde(rename = "aquaponicsystems")]
    AquaponicSystems,

    #[serde(rename = "autofabricators")]
    AutoFabricators,

    #[serde(rename = "bioreducinglichen")]
    BioreducingLichen,

    #[serde(rename = "computercomponents")]
    ComputerComponents,

    #[serde(rename = "hesuits")]
    HESuits,

    #[serde(rename = "hardwarediagnosticsensor")]
    HardwareDiagnosticSensor,

    #[serde(rename = "landenrichmentsystems")]
    LandEnrichmentSystems,

    #[serde(rename = "medialdiagnosticequipment")]
    MedialDiagnosticEquipment,

    #[serde(rename = "microcontrollers")]
    MicroControllers,

    #[serde(rename = "muonimager")]
    MuonImager,

    #[serde(rename = "nanobreakers")]
    Nanobreakers,

    #[serde(rename = "resonatingseparators")]
    ResonatingSeparators,

    #[serde(rename = "robotics")]
    Robotics,

    #[serde(rename = "structuralregulators")]
    StructuralRegulators,

    #[serde(rename = "telemetrysuite")]
    TelemetrySuite,

    // Textiles
    #[serde(rename = "conductivefabrics")]
    ConductiveFabrics,

    #[serde(rename = "leather")]
    Leather,

    #[serde(rename = "militarygradefabrics")]
    MilitaryGradeFabrics,

    #[serde(rename = "naturalfabrics")]
    NaturalFabrics,

    #[serde(rename = "syntheticfabrics")]
    SyntheticFabrics,

    // Waste
    #[serde(rename = "biowaste")]
    Biowaste,

    #[serde(rename = "chemicalwaste")]
    ChemicalWaste,

    #[serde(rename = "scrap")]
    Scrap,

    #[serde(rename = "toxicwaste")]
    ToxicWaste,

    // Weapons
    #[serde(rename = "battleweapons")]
    BattleWeapons,

    #[serde(rename = "landmines")]
    Landmines,

    #[serde(rename = "nonlethalweapons")]
    NonLethalWeapons,

    #[serde(rename = "personalweapons")]
    PersonalWeapons,

    #[serde(rename = "reactivearmour")]
    ReactiveArmour,

    // Rare
    #[serde(rename = "jaquesquinentianstill")]
    JaquesQuinentianStill,

    #[serde(rename = "kinagoviolins")]
    KinagoViolins,

    #[serde(rename = "apavietii")]
    ApaVietii,

    #[serde(rename = "geawendancedust")]
    GeawenDanceDust,

    #[serde(rename = "vanayequiceratomorphafur")]
    VanayequiCeratomorphaFur,

    #[serde(rename = "karetiicouture")]
    KaretiiCouture,

    #[serde(rename = "mukusubiichitinos")]
    MukusubiiChitinos,

    #[serde(rename = "ultracompactprocessorprototypes")]
    UltraCompactProcessorPrototypes,

    #[serde(rename = "eleuthermals")]
    EleuThermals,

    #[serde(rename = "kamorinhistoricweapons")]
    KamorinHistoricWeapons,

    #[serde(rename = "ceremonialheiketea")]
    CeremonialHeikeTea,

    #[serde(rename = "vidavantianlace")]
    VidavantianLace,

    #[serde(rename = "kachiriginfilterleeches")]
    KachiriginFilterLeeches,

    #[serde(rename = "lyraeweed")]
    LyraeWeed,

    #[serde(rename = "galactictravelguide")]
    GalacticTravelGuide,

    #[serde(rename = "harmasilversearum")]
    HarmaSilverSeaRum,

    #[serde(rename = "ngadandarifireopals")]
    NgadandariFireOpals,

    #[serde(rename = "alyabodysoap")]
    AlyaBodySoap,

    #[serde(rename = "helvetitjpearls")]
    HelvetitjPearls,

    #[serde(rename = "ochoengchillies")]
    OchoengChillies,

    #[serde(rename = "onionheadbetastrain")]
    OnionheadBetaStrain,

    #[serde(rename = "kamitracigars")]
    KamitraCigars,

    #[serde(rename = "njangarisaddles")]
    NjangariSaddles,

    #[serde(rename = "hiporganophosphates")]
    HipOrganophosphates,

    #[serde(rename = "gilyasignatureweapons")]
    GilyaSignatureWeapons,

    #[serde(rename = "hr7221wheat")]
    HR7221Wheat,

    #[serde(rename = "wheemetewheatcakes")]
    WheemeteWheatCakes,

    #[serde(rename = "rajukrumultistoves")]
    RajukruMultiStoves,

    #[serde(rename = "nanomedicines")]
    Nanomedicines,

    #[serde(rename = "noneuclidianexotanks")]
    NonEuclidianExotanks,

    #[serde(rename = "ngunamodernantiques")]
    NgunaModernAntiques,

    #[serde(rename = "xihebiomorphiccompanions")]
    XiheBiomorphicCompanions,

    #[serde(rename = "esusekucaviar")]
    EsusekuCaviar,

    #[serde(rename = "orrerianviciousbrew")]
    OrrerianViciousBrew,

    #[serde(rename = "vherculisbodyrub")]
    VHerculisBodyRub,

    #[serde(rename = "voidextractcoffee")]
    VoidExtractCoffee,

    #[serde(rename = "uszaiantreegrub")]
    UszaianTreeGrub,

    #[serde(rename = "haidenblackbrew")]
    HaidenBlackBrew,

    #[serde(rename = "motronaexperiencejelly")]
    MotronaExperienceJelly,

    #[serde(rename = "jaradharrepuzzlebox")]
    JaradharrePuzzleBox,

    #[serde(rename = "personalgifts")]
    PersonalGifts,

    #[serde(rename = "mulachigiantfungus")]
    MulachiGiantFungus,

    #[serde(rename = "ltthypersweet")]
    LTTHyperSweet,

    #[serde(rename = "medbstarlube")]
    MedbStarlube,

    #[serde(rename = "giantverrix")]
    GiantVerrix,

    #[serde(rename = "hip118311swarm")]
    HIP118311Swarm,

    #[serde(rename = "disomacorn")]
    DisoMaCorn,

    #[serde(rename = "lavianbrandy")]
    LavianBrandy,

    #[serde(rename = "azuremilk")]
    AzureMilk,

    #[serde(rename = "leestianeviljuice")]
    LeestianEvilJuice,

    #[serde(rename = "coquimspongiformvictuals")]
    CoquimSpongiformVictuals,

    #[serde(rename = "leatheryeggs")]
    LeatheryEggs,

    #[serde(rename = "shanscharisorchid")]
    ShansCharisOrchid,

    #[serde(rename = "konggaale")]
    KonggaAle,

    #[serde(rename = "vegaslimweed")]
    VegaSlimweed,

    #[serde(rename = "tiolcewaste2pasteunits")]
    TiolceWaste2PasteUnits,

    #[serde(rename = "ophiuchexinoartefacts")]
    OphiuchExinoArtefacts,

    #[serde(rename = "altairianskin")]
    AltairianSkin,

    #[serde(rename = "aganipperush")]
    AganippeRush,

    #[serde(rename = "cd75kittenbrandcoffee")]
    CD75KittenBrandCoffee,

    #[serde(rename = "havasupaidreamcatcher")]
    HavasupaiDreamCatcher,

    #[serde(rename = "eraninpearlwhisky")]
    EraninPearlWhisky,

    #[serde(rename = "pavoniseargrubs")]
    PavonisEarGrubs,

    #[serde(rename = "onionheadalphastrain")]
    OnionheadAlphaStrain,

    #[serde(rename = "indibourbon")]
    IndiBourbon,

    #[serde(rename = "bakedgreebles")]
    BakedGreebles,

    #[serde(rename = "karsukilocusts")]
    KarsukiLocusts,

    #[serde(rename = "masterchefs")]
    MasterChefs,

    #[serde(rename = "yasokondileaf")]
    YasoKondiLeaf,

    #[serde(rename = "burnhambiledistillate")]
    BurnhamBileDistillate,

    #[serde(rename = "thehuttonmug")]
    TheHuttonMug,

    #[serde(rename = "centaurimegagin")]
    CentauriMegaGin,

    #[serde(rename = "utgaroarmillennialeggs")]
    UtgaroarMillennialEggs,

    #[serde(rename = "soontillrelics")]
    SoontillRelics,

    #[serde(rename = "zeesszeantgrubglue")]
    ZeesszeAntGrubGlue,

    #[serde(rename = "thewatersofshintara")]
    TheWatersOfShintara,

    #[serde(rename = "baltahsinevacuumkrill")]
    BaltahsineVacuumKrill,

    #[serde(rename = "sanumadecorativemeat")]
    SanumaDecorativeMeat,

    #[serde(rename = "giantirukamasnails")]
    GiantIrukamaSnails,

    #[serde(rename = "anduligafireworks")]
    AnduligaFireWorks,

    #[serde(rename = "crystallinespheres")]
    CrystallineSpheres,

    #[serde(rename = "pantaaprayersticks")]
    PantaaPrayerSticks,

    #[serde(rename = "chieridanimarinepaste")]
    ChiEridaniMarinePaste,

    #[serde(rename = "ethgrezeteabuds")]
    EthgrezeTeaBuds,

    #[serde(rename = "deltaphoenicispalms")]
    DeltaPhoenicisPalms,

    #[serde(rename = "tarachspice")]
    TarachSpice,

    #[serde(rename = "wulpahyperboresystems")]
    WulpaHyperboreSystems,

    #[serde(rename = "livehecateseaworms")]
    LiveHecateSeaWorms,

    #[serde(rename = "korokungpellets")]
    KoroKungPellets,

    #[serde(rename = "bastsnakegin")]
    BastSnakeGin,

    #[serde(rename = "terramaterbloodbores")]
    TerraMaterBloodBores,

    #[serde(rename = "wuthielokufroth")]
    WuthieloKuFroth,

    #[serde(rename = "honestypills")]
    HonestyPills,

    #[serde(rename = "cromsilverfesh")]
    CromSilverFesh,

    #[serde(rename = "borasetanipathogenetics")]
    BorasetaniPathogenetics,

    #[serde(rename = "cetirabbits")]
    CetiRabbits,

    #[serde(rename = "aepyornisegg")]
    AepyornisEgg,

    #[serde(rename = "uzumokulowgwings")]
    UzumokuLowGWings,

    #[serde(rename = "cherbonesbloodcrystals")]
    CherbonesBloodCrystals,

    #[serde(rename = "toxandjivirocide")]
    ToxandjiVirocide,

    #[serde(rename = "onionhead")]
    Onionhead,

    #[serde(rename = "lucanonionhead")]
    LucanOnionhead,

    #[serde(rename = "tanmarktranquiltea")]
    TanmarkTranquilTea,

    #[serde(rename = "thrutiscream")]
    ThrutisCream,

    #[serde(rename = "alacarakmoskinart")]
    AlacarakmoSkinArt,

    #[serde(rename = "platinumalloy")]
    PlatinumAlloy,

    #[serde(rename = "mokojingbeastfeast")]
    MokojingBeastFeast,

    #[serde(rename = "edenapplesofaerial")]
    EdenApplesOfAerial,

    #[serde(rename = "chameleoncloth")]
    ChameleonCloth,

    #[serde(rename = "taurichimes")]
    TauriChimes,

    #[serde(rename = "rusanioldsmokey")]
    RusaniOldSmokey,

    #[serde(rename = "azcancriformula42")]
    AZCancriFormula42,

    #[serde(rename = "gomanyauponcoffee")]
    GomanYauponCoffee,

    #[serde(rename = "gerasiangueuzebeer")]
    GerasianGueuzeBeer,

    #[serde(rename = "jarouarice")]
    JarouaRice,

    #[serde(rename = "anynacoffee")]
    AnyNaCoffee,

    #[serde(rename = "fujintea")]
    FujinTea,

    #[serde(rename = "hip10175bushmeat")]
    HIP10175BushMeat,

    #[serde(rename = "momusbogspaniel")]
    MomusBogSpaniel,

    #[serde(rename = "witchhaulkobebeef")]
    WitchhaulKobeBeef,

    #[serde(rename = "saxonwine")]
    SaxonWine,

    #[serde(rename = "aroucaconventualsweets")]
    AroucaConventualSweets,

    #[serde(rename = "albinoquechuamammothmeat")]
    AlbinoQuechuaMammothMeat,

    #[serde(rename = "duradrives")]
    Duradrives,

    #[serde(rename = "holvaduellingblades")]
    HolvaDuellingBlades,

    #[serde(rename = "rapabaosnakeskins")]
    RapaBaoSnakeSkins,

    #[serde(rename = "wolffesh")]
    WolfFesh,

    #[serde(rename = "eshuumbrellas")]
    EshuUmbrellas,

    #[serde(rename = "neritusberries")]
    NeritusBerries,

    #[serde(rename = "jotunmookah")]
    JotunMookah,

    #[serde(rename = "chateaudeaegaeon")]
    ChateauDeAegaeon,

    #[serde(rename = "belalansrayleather")]
    BelalansRayLeather,

    #[serde(rename = "damnacarapaces")]
    DamnaCarapaces,

    #[serde(rename = "hipprotosquid")]
    HIPProtoSquid,

    #[serde(rename = "mechucoshightea")]
    MechucosHighTea,

    #[serde(rename = "deuringastruffles")]
    DeuringasTruffles,

    #[serde(rename = "bankiamphibiousleather")]
    BankiAmphibiousLeather,

    #[serde(rename = "sothiscrystallinegold")]
    SothisCrystallineGold,

    #[serde(rename = "tiegfriessynthsilk")]
    TiegfriesSynthSilk,

    #[serde(rename = "volkhabbeedrones")]
    VolkhabBeeDrones,

    #[serde(rename = "buckyballbeermats")]
    BuckyballBeerMats,

    #[serde(rename = "classifiedexperimentalequipment")]
    ClassifiedExperimentalEquipment,


    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}




// use std::str::FromStr;
// use thiserror::Error;
// use crate::from_str_deserialize_impl;
//
// #[derive(Debug)]
// #[cfg_attr(test, derive(PartialEq))]
// pub enum CommodityType {
//     Wine,
//
//     KamitraCigars,
//
//     #[cfg(not(feature = "strict"))]
//     Unknown(String),
// }
//
// #[derive(Debug, Error)]
// pub enum CommodityTypeParseError {
//     #[error("Unknown commodity: '{0}'")]
//     UnknownCommodity(String),
// }
//
// impl FromStr for CommodityType {
//     type Err = CommodityTypeParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "wine" => Ok(CommodityType::Wine),
//             "kamitracigars" => Ok(CommodityType::KamitraCigars),
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(CommodityType::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(CommodityTypeParseError::UnknownCommodity(s.to_string())),
//         }
//     }
// }
//
// from_str_deserialize_impl!(CommodityType);

