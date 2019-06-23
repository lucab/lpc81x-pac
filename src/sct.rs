#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt condition register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop condition register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start condition register"]
    pub start: START,
    _reserved0: [u8; 40usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture registers mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    _reserved1: [u8; 148usize],
    #[doc = "0xf0 - SCT event enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    #[doc = "0x100 - SCT match value register of match channels 0 to 4; REGMOD0 to REGMODE4 = 0"]
    pub match_: [MATCH; 5],
    _reserved2: [u8; 236usize],
    #[doc = "0x200 - SCT match reload value register 0 to 4 REGMOD0 = 0 to REGMODE4 = 0"]
    pub matchrel: [MATCHREL; 5],
    _reserved3: [u8; 236usize],
    #[doc = "0x300 - SCT event state register 0"]
    pub ev0_state: EV0_STATE,
    #[doc = "0x304 - SCT event control register 0"]
    pub ev0_ctrl: EV0_CTRL,
    #[doc = "0x308 - SCT event state register 1"]
    pub ev1_state: EV1_STATE,
    #[doc = "0x30c - SCT event control register 1"]
    pub ev1_ctrl: EV1_CTRL,
    #[doc = "0x310 - SCT event state register 2"]
    pub ev2_state: EV2_STATE,
    #[doc = "0x314 - SCT event control register 2"]
    pub ev2_ctrl: EV2_CTRL,
    #[doc = "0x318 - SCT event state register 3"]
    pub ev3_state: EV3_STATE,
    #[doc = "0x31c - SCT event control register 3"]
    pub ev3_ctrl: EV3_CTRL,
    #[doc = "0x320 - SCT event state register 4"]
    pub ev4_state: EV4_STATE,
    #[doc = "0x324 - SCT event control register 4"]
    pub ev4_ctrl: EV4_CTRL,
    #[doc = "0x328 - SCT event state register 5"]
    pub ev5_state: EV5_STATE,
    #[doc = "0x32c - SCT event control register 5"]
    pub ev5_ctrl: EV5_CTRL,
    _reserved4: [u8; 464usize],
    #[doc = "0x500 - SCT output 0 set register"]
    pub out0_set: OUT0_SET,
    #[doc = "0x504 - SCT output 0 clear register"]
    pub out0_clr: OUT0_CLR,
    #[doc = "0x508 - SCT output 1 set register"]
    pub out1_set: OUT1_SET,
    #[doc = "0x50c - SCT output 1 clear register"]
    pub out1_clr: OUT1_CLR,
    #[doc = "0x510 - SCT output 2 set register"]
    pub out2_set: OUT2_SET,
    #[doc = "0x514 - SCT output 2 clear register"]
    pub out2_clr: OUT2_CLR,
    #[doc = "0x518 - SCT output 3 set register"]
    pub out3_set: OUT3_SET,
    #[doc = "0x51c - SCT output 3 clear register"]
    pub out3_clr: OUT3_CLR,
}
#[doc = "SCT configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "SCT control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "SCT limit register"]
pub struct LIMIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT limit register"]
pub mod limit;
#[doc = "SCT halt condition register"]
pub struct HALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT halt condition register"]
pub mod halt;
#[doc = "SCT stop condition register"]
pub struct STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT stop condition register"]
pub mod stop;
#[doc = "SCT start condition register"]
pub struct START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT start condition register"]
pub mod start;
#[doc = "SCT counter register"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT counter register"]
pub mod count;
#[doc = "SCT state register"]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT state register"]
pub mod state;
#[doc = "SCT input register"]
pub struct INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT input register"]
pub mod input;
#[doc = "SCT match/capture registers mode register"]
pub struct REGMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match/capture registers mode register"]
pub mod regmode;
#[doc = "SCT output register"]
pub struct OUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output register"]
pub mod output;
#[doc = "SCT output counter direction control register"]
pub struct OUTPUTDIRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "SCT conflict resolution register"]
pub struct RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "SCT event enable register"]
pub struct EVEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event enable register"]
pub mod even;
#[doc = "SCT event flag register"]
pub struct EVFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "SCT conflict enable register"]
pub struct CONEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict enable register"]
pub mod conen;
#[doc = "SCT conflict flag register"]
pub struct CONFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "SCT match value register of match channels 0 to 4; REGMOD0 to REGMODE4 = 0"]
pub struct MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels 0 to 4; REGMOD0 to REGMODE4 = 0"]
pub mod match_;
#[doc = "SCT capture register of capture channel 0 to 4; REGMOD0 to REGMODE4 = 1"]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel 0 to 4; REGMOD0 to REGMODE4 = 1"]
pub mod cap;
#[doc = "SCT match reload value register 0 to 4 REGMOD0 = 0 to REGMODE4 = 0"]
pub struct MATCHREL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register 0 to 4 REGMOD0 = 0 to REGMODE4 = 0"]
pub mod matchrel;
#[doc = "SCT capture control register 0 to 4; REGMOD0 = 1 to REGMODE4 = 1"]
pub struct CAPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register 0 to 4; REGMOD0 = 1 to REGMODE4 = 1"]
pub mod capctrl;
#[doc = "SCT event state register 0"]
pub struct EV0_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 0"]
pub mod ev0_state;
#[doc = "SCT event control register 0"]
pub struct EV0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 0"]
pub mod ev0_ctrl;
#[doc = "SCT event state register 1"]
pub struct EV1_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 1"]
pub mod ev1_state;
#[doc = "SCT event control register 1"]
pub struct EV1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 1"]
pub mod ev1_ctrl;
#[doc = "SCT event state register 2"]
pub struct EV2_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 2"]
pub mod ev2_state;
#[doc = "SCT event control register 2"]
pub struct EV2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 2"]
pub mod ev2_ctrl;
#[doc = "SCT event state register 3"]
pub struct EV3_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 3"]
pub mod ev3_state;
#[doc = "SCT event control register 3"]
pub struct EV3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 3"]
pub mod ev3_ctrl;
#[doc = "SCT event state register 4"]
pub struct EV4_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 4"]
pub mod ev4_state;
#[doc = "SCT event control register 4"]
pub struct EV4_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 4"]
pub mod ev4_ctrl;
#[doc = "SCT event state register 5"]
pub struct EV5_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 5"]
pub mod ev5_state;
#[doc = "SCT event control register 5"]
pub struct EV5_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 5"]
pub mod ev5_ctrl;
#[doc = "SCT output 0 set register"]
pub struct OUT0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 0 set register"]
pub mod out0_set;
#[doc = "SCT output 0 clear register"]
pub struct OUT0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 0 clear register"]
pub mod out0_clr;
#[doc = "SCT output 1 set register"]
pub struct OUT1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 1 set register"]
pub mod out1_set;
#[doc = "SCT output 1 clear register"]
pub struct OUT1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 1 clear register"]
pub mod out1_clr;
#[doc = "SCT output 2 set register"]
pub struct OUT2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 2 set register"]
pub mod out2_set;
#[doc = "SCT output 2 clear register"]
pub struct OUT2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 2 clear register"]
pub mod out2_clr;
#[doc = "SCT output 3 set register"]
pub struct OUT3_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 3 set register"]
pub mod out3_set;
#[doc = "SCT output 3 clear register"]
pub struct OUT3_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 3 clear register"]
pub mod out3_clr;
