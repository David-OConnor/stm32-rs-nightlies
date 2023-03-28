#[doc = "Register `SECWM2R2` reader"]
pub struct R(crate::R<SECWM2R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM2R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM2R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM2R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM2R2` writer"]
pub struct W(crate::W<SECWM2R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM2R2_SPEC>;
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
impl From<crate::W<SECWM2R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM2R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP2_PSTRT` reader - PCROP2_PSTRT"]
pub type PCROP2_PSTRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCROP2_PSTRT` writer - PCROP2_PSTRT"]
pub type PCROP2_PSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM2R2_SPEC, u8, u8, 7, O>;
#[doc = "Field `PCROP2EN` reader - PCROP2EN"]
pub type PCROP2EN_R = crate::BitReader<bool>;
#[doc = "Field `PCROP2EN` writer - PCROP2EN"]
pub type PCROP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECWM2R2_SPEC, bool, O>;
#[doc = "Field `HDP2_PEND` reader - HDP2_PEND"]
pub type HDP2_PEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDP2_PEND` writer - HDP2_PEND"]
pub type HDP2_PEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM2R2_SPEC, u8, u8, 7, O>;
#[doc = "Field `HDP2EN` reader - HDP2EN"]
pub type HDP2EN_R = crate::BitReader<bool>;
#[doc = "Field `HDP2EN` writer - HDP2EN"]
pub type HDP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECWM2R2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - PCROP2_PSTRT"]
    #[inline(always)]
    pub fn pcrop2_pstrt(&self) -> PCROP2_PSTRT_R {
        PCROP2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 15 - PCROP2EN"]
    #[inline(always)]
    pub fn pcrop2en(&self) -> PCROP2EN_R {
        PCROP2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - HDP2_PEND"]
    #[inline(always)]
    pub fn hdp2_pend(&self) -> HDP2_PEND_R {
        HDP2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - HDP2EN"]
    #[inline(always)]
    pub fn hdp2en(&self) -> HDP2EN_R {
        HDP2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PCROP2_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2_pstrt(&mut self) -> PCROP2_PSTRT_W<0> {
        PCROP2_PSTRT_W::new(self)
    }
    #[doc = "Bit 15 - PCROP2EN"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2en(&mut self) -> PCROP2EN_W<15> {
        PCROP2EN_W::new(self)
    }
    #[doc = "Bits 16:22 - HDP2_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_pend(&mut self) -> HDP2_PEND_W<16> {
        HDP2_PEND_W::new(self)
    }
    #[doc = "Bit 31 - HDP2EN"]
    #[inline(always)]
    #[must_use]
    pub fn hdp2en(&mut self) -> HDP2EN_W<31> {
        HDP2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure watermak2 register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r2](index.html) module"]
pub struct SECWM2R2_SPEC;
impl crate::RegisterSpec for SECWM2R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm2r2::R](R) reader structure"]
impl crate::Readable for SECWM2R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm2r2::W](W) writer structure"]
impl crate::Writable for SECWM2R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECWM2R2 to value 0x0f00_0f00"]
impl crate::Resettable for SECWM2R2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_0f00;
}
