#[doc = "Register `PR2` reader"]
pub struct R(crate::R<PR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR2` writer"]
pub struct W(crate::W<PR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR2_SPEC>;
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
impl From<crate::W<PR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIF33` reader - Configurable event inputs x+32 Pending bit."]
pub type PIF33_R = crate::BitReader<bool>;
#[doc = "Field `PIF33` writer - Configurable event inputs x+32 Pending bit."]
pub type PIF33_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR2_SPEC, bool, O>;
#[doc = "Field `PIF40_41` reader - Configurable event inputs x+32 Pending bit."]
pub type PIF40_41_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIF40_41` writer - Configurable event inputs x+32 Pending bit."]
pub type PIF40_41_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif40_41(&self) -> PIF40_41_R {
        PIF40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif33(&mut self) -> PIF33_W<1> {
        PIF33_W::new(self)
    }
    #[doc = "Bits 8:9 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif40_41(&mut self) -> PIF40_41_W<8> {
        PIF40_41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](index.html) module"]
pub struct PR2_SPEC;
impl crate::RegisterSpec for PR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr2::R](R) reader structure"]
impl crate::Readable for PR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr2::W](W) writer structure"]
impl crate::Writable for PR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
