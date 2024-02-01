#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SourceAddress {
    /// Engine 1.
    ///
    /// The #1 on the Engine CA is to identify that this is the first PA being used
    /// for the particular function, Engine. It may only be used for the NAME Function
    /// of 0, Function Instance 0, and an ecu instance of 0, which is commonly know
    /// as the “first engine”.
    Engine1,
    /// Engine 2.
    Engine2, // 0x1
    /// Turbocharger.
    Turbocharger, // 0x2
    /// Transmission #1.
    Transmission1, // 0x3
    /// Transmission #2.
    Transmission2, // 0x4
    /// Shift Console - Primary.
    ShiftConsolePrimary, // 0x5
    /// Shift Console - Secondary.
    ShiftConsoleSecondary, // 0x6
    /// Power TakeOff - (Main or Rear).
    PowerTakeOffMainRear, // 0x7
    /// Axle - Steering.
    AxleSteering, // 0x8
    /// Axle - Drive #1.
    AxleDrive1, // 0x9
    /// Axle - Drive #2.
    AxleDrive2, // 0xa
    /// Brakes - System Controller.
    BrakesSystemController, // 0xb
    /// Brakes - Steer Axle.
    BrakesSteerAxle, // 0xc
    /// Brakes - Drive Axle #1.
    BrakesDriveAxle1, // 0xd
    /// Brakes - Drive Axle #2.
    BrakesDriveAxle2, // 0xe
    /// Retarder - Engine.
    RetarderEngine, // 0xf
    /// Retarder - Driveline.
    RetarderDriveline, // 0x10
    /// Cruise Control.
    CruiseControl, // 0x11
    /// Fuel System.
    FuelSystem, // 0x12
    /// Steering Controller.
    SteeringController, // 0x13
    /// Suspension - Steer Axle.
    SuspensionSteerAxle, // 0x14
    /// Suspension - Drive Axle #1.
    SuspensionDriveAxle1, // 0x15
    /// Suspension - Drive Axle #2.
    SuspensionDriveAxle2, // 0x16
    /// Instrument Cluster #1.
    InstrumentCluster1, // 0x17
    /// Trip Recorder.
    TripRecorder, // 0x18
    /// Passenger-Operator Climate Control #1.
    PassengerOperatorClimateControl1, // 0x19
    /// Alternator/Electrical Charging System.
    AlternatorElectricalChargingSystem, // 0x1a
    /// Aerodynamic Control.
    AerodynamicControl, // 0x1b
    /// Vehicle Navigation.
    VehicleNavigation, // 0x1c
    /// Vehicle Security.
    VehicleSecurity, // 0x1d
    /// Electrical System.
    ElectricalSystem, // 0x1e
    /// Starter System.
    StarterSystem, // 0x1f
    /// Tractor-Trailer Bridge #1.
    TractorTrailerBridge1, // 0x20
    /// Body Controller.
    BodyController, // 0x21
    /// Auxiliary Valve Control or Engine Air System Valve Control.
    AuxiliaryValveControl, // 0x22
    /// Hitch Control.
    HitchControl, // 0x23
    /// Power TakeOff (Front or Secondary).
    PowerTakeOffFrontSecondary, // 0x24
    /// Off Vehicle Gateway.
    OffVehicleGateway, // 0x25
    /// Virtual Terminal (in cab).
    VirtualTerminalInCab, // 0x26
    /// Management Computer #1.
    ///
    /// The first Management Computer - may only be used for the NAME Function of
    /// 30, Function Instance 0, and an ecu instance of 0.
    ManagementComputer1,
    /// Cab Display #1.
    ///
    /// The first Cab Display - may only be used for the NAME Function of 60, Function
    /// Instance 0, and an ecu instance of 0.
    CabDisplay1,
    /// Retarder, Exhaust, Engine #1.
    RetarderExhaustEngine1, // 0x29
    /// Headway Controller.
    HeadwayController, // 0x2a
    /// On-Board Diagnostic Unit.
    OnBoardDiagnosticUnit, // 0x2b
    /// Retarder, Exhaust, Engine #2.
    RetarderExhaustEngine2, // 0x2c
    /// Endurance Braking System.
    EnduranceBrakingSystem, // 0x2d
    /// Hydraulic Pump Controller.
    HydraulicPumpController, // 0x2e
    /// Suspension - System Controller #1.
    SuspensionSystemController1, // 0x2f
    /// Pneumatic - System Controller.
    PneumaticSystemController, // 0x30
    /// Cab Controller - Primary.
    CabControllerPrimary, // 0x31
    /// Cab Controller - Secondary.
    CabControllerSecondary, // 0x32
    /// Tire Pressure Controller.
    TirePressureController, // 0x33
    /// Ignition Control Module #1.
    IgnitionControlModule1, // 0x34
    /// Ignition Control Module #2.
    IgnitionControlModule2, // 0x35
    /// Seat Control #1.
    SeatControl1, // 0x36
    /// Lighting - Operator Controls.
    LightingOperatorControls, // 0x37
    /// Rear Axle Steering Controller #1.
    RearAxleSteeringController1, // 0x38
    /// Water Pump Controller.
    WaterPumpController, // 0x39
    /// Passenger-Operator Climate Control #2.
    PassengerOperatorClimateControl2, // 0x3a
    /// Transmission Display - Primary.
    TransmissionDisplayPrimary, // 0x3b
    /// Transmission Display - Secondary.
    TransmissionDisplaySecondary, // 0x3c
    /// Exhaust Emission Controller.
    ExhaustEmissionController, // 0x3d
    /// Vehicle Dynamic Stability Controller.
    VehicleDynamicStabilityController, // 0x3e
    /// Oil Sensor.
    OilSensor, // 0x3f
    /// Suspension - System Controller #2.
    SuspensionSystemController2, // 0x40
    /// Information System Controller #1.
    InformationSystemController1, // 0x41
    /// Ramp Control.
    RampControl, // 0x42
    /// Clutch/Converter Unit.
    ClutchConverterUnit, // 0x43
    /// Auxiliary Heater #1.
    AuxiliaryHeater1, // 0x44
    /// Auxiliary Heater #2.
    AuxiliaryHeater2, // 0x45
    /// Engine Valve Controller.
    EngineValveController, // 0x46
    /// Chassis Controller #1.
    ChassisController1, // 0x47
    /// Chassis Controller #2.
    ChassisController2, // 0x48
    /// Propulsion Battery Charger.
    PropulsionBatteryCharger, // 0x49
    /// Communications Unit, Cellular.
    CommunicationsUnitCellular, // 0x4a
    /// Communications Unit, Satellite.
    CommunicationsUnitSatellite, // 0x4b
    /// Communications Unit, Radio.
    CommunicationsUnitRadio, // 0x4c
    /// Steering Column Unit.
    SteeringColumnUnit, // 0x4d
    /// Fan Drive Controller.
    FanDriveController, // 0x4e
    /// Seat Control #2.
    SeatControl2, // 0x4f
    /// Parking brake controller.
    ParkingBrakeController, // 0x50
    /// Aftertreatment #1 system gas intake.
    Aftertreatment1SystemGasIntake, // 0x51
    /// Aftertreatment #1 system gas outlet.
    Aftertreatment1SystemGasOutlet, // 0x52
    /// Safety Restraint System.
    SafetyRestraintSystem, // 0x53
    /// Cab Display #2.
    CabDisplay2, // 0x54
    /// Diesel Particulate Filter Controller.
    DieselParticulateFilterController, // 0x55
    /// Aftertreatment #2 system gas intake.
    Aftertreatment2SystemGasIntake, // 0x56
    /// Aftertreatment #2 system gas outlet.
    Aftertreatment2SystemGasOutlet, // 0x57
    /// Safety Restraint System #2.
    SafetyRestraintSystem2, // 0x58
    /// Atmospheric Sensor.
    AtmosphericSensor, // 0x59
    /// Powertrain Control Module.
    PowertrainControlModule, // 0x5a
    /// Power Systems Manager.
    PowerSystemsManager, // 0x5b
    /// Engine Injection Control Module.
    EngineInjectionControlModule, // 0x5c
    /// Fire Protection System.
    FireProtectionSystem, // 0x5d
    /// Driver Impairment Device.
    DriverImpairmentDevice, // 0x5e
    /// Supply Equipment Communication Controller (SECC).
    SupplyEquipmentCommunicationController, // 0x5f
    /// Vehicle Adapter Communication Controller (VACC).
    VehicleAdapterCommunicationController, // 0x60
    /// Fuel Cell System.
    FuelCellSystem, // 0x61
    /// SAE reserved.
    SAEReserved(u8), // 0x62 - 0x7f
    /// Dynamic address assignment.
    Dynamic(u8), // 0x80 - 0xf7
    /// File Server / Printer.
    FileServerPrinter, // 0xf8
    /// Off Board Diagnostic-Service Tool #1.
    OffBoardDiagnosticServiceTool1, // 0xf9
    /// Off Board Diagnostic-Service Tool #2.
    OffBoardDiagnosticServiceTool2, // 0xfa
    /// On-Board Data Logger
    OnBoardDataLogger, // 0xfb
    /// Experimental.
    Experimental, // 0xfc
    /// OEM Reserved.
    OEMReserved, // 0xfd
    /// Address claim failed.
    Null, // 0xfe
    /// Global address.
    Global, // 0xff
}

impl From<u8> for SourceAddress {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SourceAddress::Engine1,
            0x1 => SourceAddress::Engine2,
            0x2 => SourceAddress::Turbocharger,
            0x3 => SourceAddress::Transmission1,
            0x4 => SourceAddress::Transmission2,
            0x5 => SourceAddress::ShiftConsolePrimary,
            0x6 => SourceAddress::ShiftConsoleSecondary,
            0x7 => SourceAddress::PowerTakeOffMainRear,
            0x8 => SourceAddress::AxleSteering,
            0x9 => SourceAddress::AxleDrive1,
            0xa => SourceAddress::AxleDrive2,
            0xb => SourceAddress::BrakesSystemController,
            0xc => SourceAddress::BrakesSteerAxle,
            0xd => SourceAddress::BrakesDriveAxle1,
            0xe => SourceAddress::BrakesDriveAxle2,
            0xf => SourceAddress::RetarderEngine,
            0x10 => SourceAddress::RetarderDriveline,
            0x11 => SourceAddress::CruiseControl,
            0x12 => SourceAddress::FuelSystem,
            0x13 => SourceAddress::SteeringController,
            0x14 => SourceAddress::SuspensionSteerAxle,
            0x15 => SourceAddress::SuspensionDriveAxle1,
            0x16 => SourceAddress::SuspensionDriveAxle2,
            0x17 => SourceAddress::InstrumentCluster1,
            0x18 => SourceAddress::TripRecorder,
            0x19 => SourceAddress::PassengerOperatorClimateControl1,
            0x1a => SourceAddress::AlternatorElectricalChargingSystem,
            0x1b => SourceAddress::AerodynamicControl,
            0x1c => SourceAddress::VehicleNavigation,
            0x1d => SourceAddress::VehicleSecurity,
            0x1e => SourceAddress::ElectricalSystem,
            0x1f => SourceAddress::StarterSystem,
            0x20 => SourceAddress::TractorTrailerBridge1,
            0x21 => SourceAddress::BodyController,
            0x22 => SourceAddress::AuxiliaryValveControl,
            0x23 => SourceAddress::HitchControl,
            0x24 => SourceAddress::PowerTakeOffFrontSecondary,
            0x25 => SourceAddress::OffVehicleGateway,
            0x26 => SourceAddress::VirtualTerminalInCab,
            0x27 => SourceAddress::ManagementComputer1,
            0x28 => SourceAddress::CabDisplay1,
            0x29 => SourceAddress::RetarderExhaustEngine1,
            0x2a => SourceAddress::HeadwayController,
            0x2b => SourceAddress::OnBoardDiagnosticUnit,
            0x2c => SourceAddress::RetarderExhaustEngine2,
            0x2d => SourceAddress::EnduranceBrakingSystem,
            0x2e => SourceAddress::HydraulicPumpController,
            0x2f => SourceAddress::SuspensionSystemController1,
            0x30 => SourceAddress::PneumaticSystemController,
            0x31 => SourceAddress::CabControllerPrimary,
            0x32 => SourceAddress::CabControllerSecondary,
            0x33 => SourceAddress::TirePressureController,
            0x34 => SourceAddress::IgnitionControlModule1,
            0x35 => SourceAddress::IgnitionControlModule2,
            0x36 => SourceAddress::SeatControl1,
            0x37 => SourceAddress::LightingOperatorControls,
            0x38 => SourceAddress::RearAxleSteeringController1,
            0x39 => SourceAddress::WaterPumpController,
            0x3a => SourceAddress::PassengerOperatorClimateControl2,
            0x3b => SourceAddress::TransmissionDisplayPrimary,
            0x3c => SourceAddress::TransmissionDisplaySecondary,
            0x3d => SourceAddress::ExhaustEmissionController,
            0x3e => SourceAddress::VehicleDynamicStabilityController,
            0x3f => SourceAddress::OilSensor,
            0x40 => SourceAddress::SuspensionSystemController2,
            0x41 => SourceAddress::InformationSystemController1,
            0x42 => SourceAddress::RampControl,
            0x43 => SourceAddress::ClutchConverterUnit,
            0x44 => SourceAddress::AuxiliaryHeater1,
            0x45 => SourceAddress::AuxiliaryHeater2,
            0x46 => SourceAddress::EngineValveController,
            0x47 => SourceAddress::ChassisController1,
            0x48 => SourceAddress::ChassisController2,
            0x49 => SourceAddress::PropulsionBatteryCharger,
            0x4a => SourceAddress::CommunicationsUnitCellular,
            0x4b => SourceAddress::CommunicationsUnitSatellite,
            0x4c => SourceAddress::CommunicationsUnitRadio,
            0x4d => SourceAddress::SteeringColumnUnit,
            0x4e => SourceAddress::FanDriveController,
            0x4f => SourceAddress::SeatControl2,
            0x50 => SourceAddress::ParkingBrakeController,
            0x51 => SourceAddress::Aftertreatment1SystemGasIntake,
            0x52 => SourceAddress::Aftertreatment1SystemGasOutlet,
            0x53 => SourceAddress::SafetyRestraintSystem,
            0x54 => SourceAddress::CabDisplay2,
            0x55 => SourceAddress::DieselParticulateFilterController,
            0x56 => SourceAddress::Aftertreatment2SystemGasIntake,
            0x57 => SourceAddress::Aftertreatment2SystemGasOutlet,
            0x58 => SourceAddress::SafetyRestraintSystem2,
            0x59 => SourceAddress::AtmosphericSensor,
            0x5a => SourceAddress::PowertrainControlModule,
            0x5b => SourceAddress::PowerSystemsManager,
            0x5c => SourceAddress::EngineInjectionControlModule,
            0x5d => SourceAddress::FireProtectionSystem,
            0x5e => SourceAddress::DriverImpairmentDevice,
            0x5f => SourceAddress::SupplyEquipmentCommunicationController,
            0x60 => SourceAddress::VehicleAdapterCommunicationController,
            0x61 => SourceAddress::FuelCellSystem,
            0x62..=0x7f => SourceAddress::SAEReserved(value),
            0x80..=0xf7 => SourceAddress::Dynamic(value),
            0xf8 => SourceAddress::FileServerPrinter,
            0xf9 => SourceAddress::OffBoardDiagnosticServiceTool1,
            0xfa => SourceAddress::OffBoardDiagnosticServiceTool2,
            0xfb => SourceAddress::OnBoardDataLogger,
            0xfc => SourceAddress::Experimental,
            0xfd => SourceAddress::OEMReserved,
            0xfe => SourceAddress::Null,
            0xff => SourceAddress::Global,
        }
    }
}

impl From<SourceAddress> for u8 {
    fn from(value: SourceAddress) -> Self {
        match value {
            SourceAddress::Engine1 => 0x0,
            SourceAddress::Engine2 => 0x1,
            SourceAddress::Turbocharger => 0x2,
            SourceAddress::Transmission1 => 0x3,
            SourceAddress::Transmission2 => 0x4,
            SourceAddress::ShiftConsolePrimary => 0x5,
            SourceAddress::ShiftConsoleSecondary => 0x6,
            SourceAddress::PowerTakeOffMainRear => 0x7,
            SourceAddress::AxleSteering => 0x8,
            SourceAddress::AxleDrive1 => 0x9,
            SourceAddress::AxleDrive2 => 0xa,
            SourceAddress::BrakesSystemController => 0xb,
            SourceAddress::BrakesSteerAxle => 0xc,
            SourceAddress::BrakesDriveAxle1 => 0xd,
            SourceAddress::BrakesDriveAxle2 => 0xe,
            SourceAddress::RetarderEngine => 0xf,
            SourceAddress::RetarderDriveline => 0x10,
            SourceAddress::CruiseControl => 0x11,
            SourceAddress::FuelSystem => 0x12,
            SourceAddress::SteeringController => 0x13,
            SourceAddress::SuspensionSteerAxle => 0x14,
            SourceAddress::SuspensionDriveAxle1 => 0x15,
            SourceAddress::SuspensionDriveAxle2 => 0x16,
            SourceAddress::InstrumentCluster1 => 0x17,
            SourceAddress::TripRecorder => 0x18,
            SourceAddress::PassengerOperatorClimateControl1 => 0x19,
            SourceAddress::AlternatorElectricalChargingSystem => 0x1a,
            SourceAddress::AerodynamicControl => 0x1b,
            SourceAddress::VehicleNavigation => 0x1c,
            SourceAddress::VehicleSecurity => 0x1d,
            SourceAddress::ElectricalSystem => 0x1e,
            SourceAddress::StarterSystem => 0x1f,
            SourceAddress::TractorTrailerBridge1 => 0x20,
            SourceAddress::BodyController => 0x21,
            SourceAddress::AuxiliaryValveControl => 0x22,
            SourceAddress::HitchControl => 0x23,
            SourceAddress::PowerTakeOffFrontSecondary => 0x24,
            SourceAddress::OffVehicleGateway => 0x25,
            SourceAddress::VirtualTerminalInCab => 0x26,
            SourceAddress::ManagementComputer1 => 0x27,
            SourceAddress::CabDisplay1 => 0x28,
            SourceAddress::RetarderExhaustEngine1 => 0x29,
            SourceAddress::HeadwayController => 0x2a,
            SourceAddress::OnBoardDiagnosticUnit => 0x2b,
            SourceAddress::RetarderExhaustEngine2 => 0x2c,
            SourceAddress::EnduranceBrakingSystem => 0x2d,
            SourceAddress::HydraulicPumpController => 0x2e,
            SourceAddress::SuspensionSystemController1 => 0x2f,
            SourceAddress::PneumaticSystemController => 0x30,
            SourceAddress::CabControllerPrimary => 0x31,
            SourceAddress::CabControllerSecondary => 0x32,
            SourceAddress::TirePressureController => 0x33,
            SourceAddress::IgnitionControlModule1 => 0x34,
            SourceAddress::IgnitionControlModule2 => 0x35,
            SourceAddress::SeatControl1 => 0x36,
            SourceAddress::LightingOperatorControls => 0x37,
            SourceAddress::RearAxleSteeringController1 => 0x38,
            SourceAddress::WaterPumpController => 0x39,
            SourceAddress::PassengerOperatorClimateControl2 => 0x3a,
            SourceAddress::TransmissionDisplayPrimary => 0x3b,
            SourceAddress::TransmissionDisplaySecondary => 0x3c,
            SourceAddress::ExhaustEmissionController => 0x3d,
            SourceAddress::VehicleDynamicStabilityController => 0x3e,
            SourceAddress::OilSensor => 0x3f,
            SourceAddress::SuspensionSystemController2 => 0x40,
            SourceAddress::InformationSystemController1 => 0x41,
            SourceAddress::RampControl => 0x42,
            SourceAddress::ClutchConverterUnit => 0x43,
            SourceAddress::AuxiliaryHeater1 => 0x44,
            SourceAddress::AuxiliaryHeater2 => 0x45,
            SourceAddress::EngineValveController => 0x46,
            SourceAddress::ChassisController1 => 0x47,
            SourceAddress::ChassisController2 => 0x48,
            SourceAddress::PropulsionBatteryCharger => 0x49,
            SourceAddress::CommunicationsUnitCellular => 0x4a,
            SourceAddress::CommunicationsUnitSatellite => 0x4b,
            SourceAddress::CommunicationsUnitRadio => 0x4c,
            SourceAddress::SteeringColumnUnit => 0x4d,
            SourceAddress::FanDriveController => 0x4e,
            SourceAddress::SeatControl2 => 0x4f,
            SourceAddress::ParkingBrakeController => 0x50,
            SourceAddress::Aftertreatment1SystemGasIntake => 0x51,
            SourceAddress::Aftertreatment1SystemGasOutlet => 0x52,
            SourceAddress::SafetyRestraintSystem => 0x53,
            SourceAddress::CabDisplay2 => 0x54,
            SourceAddress::DieselParticulateFilterController => 0x55,
            SourceAddress::Aftertreatment2SystemGasIntake => 0x56,
            SourceAddress::Aftertreatment2SystemGasOutlet => 0x57,
            SourceAddress::SafetyRestraintSystem2 => 0x58,
            SourceAddress::AtmosphericSensor => 0x59,
            SourceAddress::PowertrainControlModule => 0x5a,
            SourceAddress::PowerSystemsManager => 0x5b,
            SourceAddress::EngineInjectionControlModule => 0x5c,
            SourceAddress::FireProtectionSystem => 0x5d,
            SourceAddress::DriverImpairmentDevice => 0x5e,
            SourceAddress::SupplyEquipmentCommunicationController => 0x5f,
            SourceAddress::VehicleAdapterCommunicationController => 0x60,
            SourceAddress::FuelCellSystem => 0x61,
            SourceAddress::SAEReserved(value) => value,
            SourceAddress::Dynamic(value) => value,
            SourceAddress::FileServerPrinter => 0xf8,
            SourceAddress::OffBoardDiagnosticServiceTool1 => 0xf9,
            SourceAddress::OffBoardDiagnosticServiceTool2 => 0xfa,
            SourceAddress::OnBoardDataLogger => 0xfb,
            SourceAddress::Experimental => 0xfc,
            SourceAddress::OEMReserved => 0xfd,
            SourceAddress::Null => 0xfe,
            SourceAddress::Global => 0xff,
        }
    }
}
