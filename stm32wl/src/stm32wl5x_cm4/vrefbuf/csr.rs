#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVR` reader - Voltage reference buffer mode enable"]
pub type ENVR_R = crate::BitReader<ENVR_A>;
#[doc = "Voltage reference buffer mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENVR_A {
    #[doc = "0: Internal voltage reference mode disable (external voltage reference mode)"]
    Disabled = 0,
    #[doc = "1: Internal voltage reference mode (reference buffer enable or hold mode) enable"]
    Enabled = 1,
}
impl From<ENVR_A> for bool {
    #[inline(always)]
    fn from(variant: ENVR_A) -> Self {
        variant as u8 != 0
    }
}
impl ENVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENVR_A {
        match self.bits {
            false => ENVR_A::Disabled,
            true => ENVR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENVR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENVR_A::Enabled
    }
}
#[doc = "Field `ENVR` writer - Voltage reference buffer mode enable"]
pub type ENVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, ENVR_A, O>;
impl<'a, const O: u8> ENVR_W<'a, O> {
    #[doc = "Internal voltage reference mode disable (external voltage reference mode)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENVR_A::Disabled)
    }
    #[doc = "Internal voltage reference mode (reference buffer enable or hold mode) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENVR_A::Enabled)
    }
}
#[doc = "Field `HIZ` reader - High impedance mode"]
pub type HIZ_R = crate::BitReader<HIZ_A>;
#[doc = "High impedance mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIZ_A {
    #[doc = "0: VREF+ pin is internally connected to the voltage reference buffer output"]
    Connected = 0,
    #[doc = "1: VREF+ pin is high impedance"]
    HighZ = 1,
}
impl From<HIZ_A> for bool {
    #[inline(always)]
    fn from(variant: HIZ_A) -> Self {
        variant as u8 != 0
    }
}
impl HIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIZ_A {
        match self.bits {
            false => HIZ_A::Connected,
            true => HIZ_A::HighZ,
        }
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == HIZ_A::Connected
    }
    #[doc = "Checks if the value of the field is `HighZ`"]
    #[inline(always)]
    pub fn is_high_z(&self) -> bool {
        *self == HIZ_A::HighZ
    }
}
#[doc = "Field `HIZ` writer - High impedance mode"]
pub type HIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, HIZ_A, O>;
impl<'a, const O: u8> HIZ_W<'a, O> {
    #[doc = "VREF+ pin is internally connected to the voltage reference buffer output"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(HIZ_A::Connected)
    }
    #[doc = "VREF+ pin is high impedance"]
    #[inline(always)]
    pub fn high_z(self) -> &'a mut W {
        self.variant(HIZ_A::HighZ)
    }
}
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub type VRS_R = crate::BitReader<VRS_A>;
#[doc = "Voltage reference scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRS_A {
    #[doc = "0: Voltage reference set to VREF_OUT1 (around 2.048 V)"]
    V2048 = 0,
    #[doc = "1: Voltage reference set to VREF_OUT2 (around 2.5 V)"]
    V25 = 1,
}
impl From<VRS_A> for bool {
    #[inline(always)]
    fn from(variant: VRS_A) -> Self {
        variant as u8 != 0
    }
}
impl VRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRS_A {
        match self.bits {
            false => VRS_A::V2048,
            true => VRS_A::V25,
        }
    }
    #[doc = "Checks if the value of the field is `V2048`"]
    #[inline(always)]
    pub fn is_v2_048(&self) -> bool {
        *self == VRS_A::V2048
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == VRS_A::V25
    }
}
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub type VRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, VRS_A, O>;
impl<'a, const O: u8> VRS_W<'a, O> {
    #[doc = "Voltage reference set to VREF_OUT1 (around 2.048 V)"]
    #[inline(always)]
    pub fn v2_048(self) -> &'a mut W {
        self.variant(VRS_A::V2048)
    }
    #[doc = "Voltage reference set to VREF_OUT2 (around 2.5 V)"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(VRS_A::V25)
    }
}
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VRR_R = crate::BitReader<VRR_A>;
#[doc = "Voltage reference buffer ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRR_A {
    #[doc = "0: The voltage reference buffer output is not ready"]
    NotReady = 0,
    #[doc = "1: The voltage reference buffer output reached the requested level"]
    Ready = 1,
}
impl From<VRR_A> for bool {
    #[inline(always)]
    fn from(variant: VRR_A) -> Self {
        variant as u8 != 0
    }
}
impl VRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRR_A {
        match self.bits {
            false => VRR_A::NotReady,
            true => VRR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VRR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VRR_A::Ready
    }
}
impl R {
    #[doc = "Bit 0 - Voltage reference buffer mode enable"]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> ENVR_W<0> {
        ENVR_W::new(self)
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<1> {
        HIZ_W::new(self)
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VRS_W<2> {
        VRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x02"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
