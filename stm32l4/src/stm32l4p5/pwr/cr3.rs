#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1"]
pub type EWUP1_R = crate::BitReader<EWUP1_A>;
#[doc = "Enable Wakeup pin WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1_A {
    #[doc = "0: External Wakeup pin WKUPx is disabled"]
    Disabled = 0,
    #[doc = "1: When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register"]
    Enabled = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::Disabled,
            true => EWUP1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1_A::Enabled
    }
}
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1"]
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, EWUP1_A, O>;
impl<'a, const O: u8> EWUP1_W<'a, O> {
    #[doc = "External Wakeup pin WKUPx is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Disabled)
    }
    #[doc = "When this bit is set, the external wakeup pin WKUPx is enabled and triggers a wakeup from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WPx bit in the PWR_CR4 register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Enabled)
    }
}
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2"]
pub use EWUP1_R as EWUP2_R;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3"]
pub use EWUP1_R as EWUP3_R;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4"]
pub use EWUP1_R as EWUP4_R;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5"]
pub use EWUP1_R as EWUP5_R;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2"]
pub use EWUP1_W as EWUP2_W;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3"]
pub use EWUP1_W as EWUP3_W;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4"]
pub use EWUP1_W as EWUP4_W;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5"]
pub use EWUP1_W as EWUP5_W;
#[doc = "Field `RRS` reader - SRAM2 retention in Standby mode"]
pub type RRS_R = crate::FieldReader<u8, RRS_A>;
#[doc = "SRAM2 retention in Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RRS_A {
    #[doc = "0: SRAM2 is powered off in Standby mode (SRAM2 content is lost)"]
    PoweredOff = 0,
    #[doc = "1: Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)"]
    PoweredOn = 1,
    #[doc = "2: Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)"]
    PartialPoweredOn = 2,
}
impl From<RRS_A> for u8 {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as _
    }
}
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RRS_A> {
        match self.bits {
            0 => Some(RRS_A::PoweredOff),
            1 => Some(RRS_A::PoweredOn),
            2 => Some(RRS_A::PartialPoweredOn),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PoweredOff`"]
    #[inline(always)]
    pub fn is_powered_off(&self) -> bool {
        *self == RRS_A::PoweredOff
    }
    #[doc = "Checks if the value of the field is `PoweredOn`"]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == RRS_A::PoweredOn
    }
    #[doc = "Checks if the value of the field is `PartialPoweredOn`"]
    #[inline(always)]
    pub fn is_partial_powered_on(&self) -> bool {
        *self == RRS_A::PartialPoweredOn
    }
}
#[doc = "Field `RRS` writer - SRAM2 retention in Standby mode"]
pub type RRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, RRS_A, 2, O>;
impl<'a, const O: u8> RRS_W<'a, O> {
    #[doc = "SRAM2 is powered off in Standby mode (SRAM2 content is lost)"]
    #[inline(always)]
    pub fn powered_off(self) -> &'a mut W {
        self.variant(RRS_A::PoweredOff)
    }
    #[doc = "Full SRAM2 is powered by the low-power regulator in Standby mode (SRAM2 full content is kept)"]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut W {
        self.variant(RRS_A::PoweredOn)
    }
    #[doc = "Only 4 Kbytes of SRAM2 is powered by the low-power regulator in Standby mode (4 Kbytes of SRAM2 content is kept)"]
    #[inline(always)]
    pub fn partial_powered_on(self) -> &'a mut W {
        self.variant(RRS_A::PartialPoweredOn)
    }
}
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration"]
pub type APC_R = crate::BitReader<APC_A>;
#[doc = "Apply pull-up and pull-down configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC_A {
    #[doc = "0: Configurations are not applied"]
    Disabled = 0,
    #[doc = "1: When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode"]
    Enabled = 1,
}
impl From<APC_A> for bool {
    #[inline(always)]
    fn from(variant: APC_A) -> Self {
        variant as u8 != 0
    }
}
impl APC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APC_A {
        match self.bits {
            false => APC_A::Disabled,
            true => APC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC_A::Enabled
    }
}
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration"]
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, APC_A, O>;
impl<'a, const O: u8> APC_W<'a, O> {
    #[doc = "Configurations are not applied"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APC_A::Disabled)
    }
    #[doc = "When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os will be in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during Run mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(APC_A::Enabled)
    }
}
#[doc = "Field `ENULP` reader - Enable ULP sampling"]
pub type ENULP_R = crate::BitReader<ENULP_A>;
#[doc = "Enable ULP sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENULP_A {
    #[doc = "0: Sampling disabled"]
    Disabled = 0,
    #[doc = "1: When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes"]
    Enabled = 1,
}
impl From<ENULP_A> for bool {
    #[inline(always)]
    fn from(variant: ENULP_A) -> Self {
        variant as u8 != 0
    }
}
impl ENULP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENULP_A {
        match self.bits {
            false => ENULP_A::Disabled,
            true => ENULP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENULP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENULP_A::Enabled
    }
}
#[doc = "Field `ENULP` writer - Enable ULP sampling"]
pub type ENULP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, ENULP_A, O>;
impl<'a, const O: u8> ENULP_W<'a, O> {
    #[doc = "Sampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENULP_A::Disabled)
    }
    #[doc = "When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENULP_A::Enabled)
    }
}
#[doc = "Field `DSIPDEN` reader - Enable Pull-down activation on DSI pins"]
pub type DSIPDEN_R = crate::BitReader<DSIPDEN_A>;
#[doc = "Enable Pull-down activation on DSI pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIPDEN_A {
    #[doc = "0: Pull-Down is disabled on DSI pins"]
    Disabled = 0,
    #[doc = "1: Pull-Down is enabled on DSI pins"]
    Enabled = 1,
}
impl From<DSIPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSIPDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DSIPDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIPDEN_A {
        match self.bits {
            false => DSIPDEN_A::Disabled,
            true => DSIPDEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSIPDEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSIPDEN_A::Enabled
    }
}
#[doc = "Field `DSIPDEN` writer - Enable Pull-down activation on DSI pins"]
pub type DSIPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DSIPDEN_A, O>;
impl<'a, const O: u8> DSIPDEN_W<'a, O> {
    #[doc = "Pull-Down is disabled on DSI pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DSIPDEN_A::Disabled)
    }
    #[doc = "Pull-Down is enabled on DSI pins"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DSIPDEN_A::Enabled)
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line"]
pub type EIWUL_R = crate::BitReader<EIWUL_A>;
#[doc = "Enable internal wakeup line\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL_A {
    #[doc = "0: Internal wakeup line disable"]
    Disabled = 0,
    #[doc = "1: Internal wakeup line enable"]
    Enabled = 1,
}
impl From<EIWUL_A> for bool {
    #[inline(always)]
    fn from(variant: EIWUL_A) -> Self {
        variant as u8 != 0
    }
}
impl EIWUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIWUL_A {
        match self.bits {
            false => EIWUL_A::Disabled,
            true => EIWUL_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIWUL_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIWUL_A::Enabled
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line"]
pub type EIWUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, EIWUL_A, O>;
impl<'a, const O: u8> EIWUL_W<'a, O> {
    #[doc = "Internal wakeup line disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIWUL_A::Disabled)
    }
    #[doc = "Internal wakeup line enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIWUL_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable ULP sampling"]
    #[inline(always)]
    pub fn enulp(&self) -> ENULP_R {
        ENULP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Pull-down activation on DSI pins"]
    #[inline(always)]
    pub fn dsipden(&self) -> DSIPDEN_R {
        DSIPDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<0> {
        EWUP1_W::new(self)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<1> {
        EWUP2_W::new(self)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<2> {
        EWUP3_W::new(self)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<3> {
        EWUP4_W::new(self)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<4> {
        EWUP5_W::new(self)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<8> {
        RRS_W::new(self)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<10> {
        APC_W::new(self)
    }
    #[doc = "Bit 11 - Enable ULP sampling"]
    #[inline(always)]
    #[must_use]
    pub fn enulp(&mut self) -> ENULP_W<11> {
        ENULP_W::new(self)
    }
    #[doc = "Bit 12 - Enable Pull-down activation on DSI pins"]
    #[inline(always)]
    #[must_use]
    pub fn dsipden(&mut self) -> DSIPDEN_W<12> {
        DSIPDEN_W::new(self)
    }
    #[doc = "Bit 15 - Enable internal wakeup line"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<15> {
        EIWUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x8000"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
