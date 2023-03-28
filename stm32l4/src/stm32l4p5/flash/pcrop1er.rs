#[doc = "Register `PCROP1ER` reader"]
pub struct R(crate::R<PCROP1ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1ER` writer"]
pub struct W(crate::W<PCROP1ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1ER_SPEC>;
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
impl From<crate::W<PCROP1ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1_END` reader - Bank 1 PCROP area end offset"]
pub type PCROP1_END_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PCROP1_END` writer - Bank 1 PCROP area end offset"]
pub type PCROP1_END_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PCROP1ER_SPEC, u32, u32, 17, O>;
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader<PCROP_RDP_A>;
#[doc = "PCROP area preserved when RDP level decreased\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCROP_RDP_A {
    #[doc = "0: PCROP area is not erased when the RDP level is decreased from Level 1 to Level 0"]
    Disabled = 0,
    #[doc = "1: PCROP area is erased when the RDP level is decreased from Level 1 to Level 0"]
    Enabled = 1,
}
impl From<PCROP_RDP_A> for bool {
    #[inline(always)]
    fn from(variant: PCROP_RDP_A) -> Self {
        variant as u8 != 0
    }
}
impl PCROP_RDP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCROP_RDP_A {
        match self.bits {
            false => PCROP_RDP_A::Disabled,
            true => PCROP_RDP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCROP_RDP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCROP_RDP_A::Enabled
    }
}
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCROP1ER_SPEC, PCROP_RDP_A, O>;
impl<'a, const O: u8> PCROP_RDP_W<'a, O> {
    #[doc = "PCROP area is not erased when the RDP level is decreased from Level 1 to Level 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCROP_RDP_A::Disabled)
    }
    #[doc = "PCROP area is erased when the RDP level is decreased from Level 1 to Level 0"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCROP_RDP_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:16 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W<0> {
        PCROP1_END_W::new(self)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<31> {
        PCROP_RDP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 PCROP End address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1er](index.html) module"]
pub struct PCROP1ER_SPEC;
impl crate::RegisterSpec for PCROP1ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1er::R](R) reader structure"]
impl crate::Readable for PCROP1ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1er::W](W) writer structure"]
impl crate::Writable for PCROP1ER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCROP1ER to value 0x0fff_0000"]
impl crate::Resettable for PCROP1ER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
