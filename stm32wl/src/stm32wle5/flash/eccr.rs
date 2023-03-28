#[doc = "Register `ECCR` reader"]
pub struct R(crate::R<ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCR` writer"]
pub struct W(crate::W<ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCR_SPEC>;
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
impl From<crate::W<ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type ADDR_ECC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYSF_ECC` reader - System Flash ECC fail"]
pub type SYSF_ECC_R = crate::BitReader<SYSF_ECC_A>;
#[doc = "System Flash ECC fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSF_ECC_A {
    #[doc = "0: No System Flash memory ECC fail"]
    NotInFlash = 0,
    #[doc = "1: System Flash memory ECC fail"]
    InFlash = 1,
}
impl From<SYSF_ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SYSF_ECC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSF_ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSF_ECC_A {
        match self.bits {
            false => SYSF_ECC_A::NotInFlash,
            true => SYSF_ECC_A::InFlash,
        }
    }
    #[doc = "Checks if the value of the field is `NotInFlash`"]
    #[inline(always)]
    pub fn is_not_in_flash(&self) -> bool {
        *self == SYSF_ECC_A::NotInFlash
    }
    #[doc = "Checks if the value of the field is `InFlash`"]
    #[inline(always)]
    pub fn is_in_flash(&self) -> bool {
        *self == SYSF_ECC_A::InFlash
    }
}
#[doc = "Field `ECCCIE` reader - ECC correction interrupt enable"]
pub type ECCCIE_R = crate::BitReader<ECCCIE_A>;
#[doc = "ECC correction interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCIE_A {
    #[doc = "0: ECCC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ECCC interrupt enabled"]
    Enabled = 1,
}
impl From<ECCCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ECCCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCCIE_A {
        match self.bits {
            false => ECCCIE_A::Disabled,
            true => ECCCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCCIE_A::Enabled
    }
}
#[doc = "Field `ECCCIE` writer - ECC correction interrupt enable"]
pub type ECCCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCCIE_A, O>;
impl<'a, const O: u8> ECCCIE_W<'a, O> {
    #[doc = "ECCC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCCIE_A::Disabled)
    }
    #[doc = "ECCC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCCIE_A::Enabled)
    }
}
#[doc = "Field `ECCC` reader - ECC correction"]
pub type ECCC_R = crate::BitReader<ECCCR_A>;
#[doc = "ECC correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCR_A {
    #[doc = "0: ECC error corrected"]
    NoEvent = 0,
    #[doc = "1: No ECC error corrected"]
    Event = 1,
}
impl From<ECCCR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCCR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCCR_A {
        match self.bits {
            false => ECCCR_A::NoEvent,
            true => ECCCR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ECCCR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == ECCCR_A::Event
    }
}
#[doc = "ECC correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCCW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<ECCCW_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCC` writer - ECC correction"]
pub type ECCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCCW_AW, O>;
impl<'a, const O: u8> ECCC_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCCW_AW::Clear)
    }
}
#[doc = "Field `ECCD` reader - ECC detection"]
pub type ECCD_R = crate::BitReader<ECCDR_A>;
#[doc = "ECC detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDR_A {
    #[doc = "0: Two ECC errors detected"]
    NoEvent = 0,
    #[doc = "1: No two ECC errors detected"]
    Event = 1,
}
impl From<ECCDR_A> for bool {
    #[inline(always)]
    fn from(variant: ECCDR_A) -> Self {
        variant as u8 != 0
    }
}
impl ECCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCDR_A {
        match self.bits {
            false => ECCDR_A::NoEvent,
            true => ECCDR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == ECCDR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == ECCDR_A::Event
    }
}
#[doc = "ECC detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCDW_AW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<ECCDW_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCDW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCD` writer - ECC detection"]
pub type ECCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, ECCDW_AW, O>;
impl<'a, const O: u8> ECCD_W<'a, O> {
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ECCDW_AW::Clear)
    }
}
impl R {
    #[doc = "Bits 0:16 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 20 - System Flash ECC fail"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecccie(&mut self) -> ECCCIE_W<24> {
        ECCCIE_W::new(self)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<30> {
        ECCC_W::new(self)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<31> {
        ECCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash ECC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr](index.html) module"]
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccr::R](R) reader structure"]
impl crate::Readable for ECCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccr::W](W) writer structure"]
impl crate::Writable for ECCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
