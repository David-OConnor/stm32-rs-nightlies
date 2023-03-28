#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITR_RMP` reader - Internal trigger remap"]
pub type ITR_RMP_R = crate::BitReader<bool>;
#[doc = "Field `ITR_RMP` writer - Internal trigger remap"]
pub type ITR_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
#[doc = "Field `ETR_RMP` reader - External trigger remap"]
pub type ETR_RMP_R = crate::BitReader<bool>;
#[doc = "Field `ETR_RMP` writer - External trigger remap"]
pub type ETR_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
#[doc = "Field `TI4_RMP` reader - Input capture 4 remap"]
pub type TI4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI4_RMP` writer - Input capture 4 remap"]
pub type TI4_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Internal trigger remap"]
    #[inline(always)]
    pub fn itr_rmp(&self) -> ITR_RMP_R {
        ITR_RMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal trigger remap"]
    #[inline(always)]
    #[must_use]
    pub fn itr_rmp(&mut self) -> ITR_RMP_W<0> {
        ITR_RMP_W::new(self)
    }
    #[doc = "Bit 1 - External trigger remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<1> {
        ETR_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 4 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<2> {
        TI4_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
