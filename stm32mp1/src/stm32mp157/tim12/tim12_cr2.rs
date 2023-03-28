#[doc = "Register `TIM12_CR2` reader"]
pub struct R(crate::R<TIM12_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM12_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM12_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM12_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM12_CR2` writer"]
pub struct W(crate::W<TIM12_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM12_CR2_SPEC>;
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
impl From<crate::W<TIM12_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM12_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMS` reader - MMS"]
pub type MMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS` writer - MMS"]
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM12_CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `TI1S` reader - TI1S"]
pub type TI1S_R = crate::BitReader<bool>;
#[doc = "Field `TI1S` writer - TI1S"]
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM12_CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - MMS"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    #[doc = "Bit 7 - TI1S"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM12 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_cr2](index.html) module"]
pub struct TIM12_CR2_SPEC;
impl crate::RegisterSpec for TIM12_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim12_cr2::R](R) reader structure"]
impl crate::Readable for TIM12_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim12_cr2::W](W) writer structure"]
impl crate::Writable for TIM12_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM12_CR2 to value 0"]
impl crate::Resettable for TIM12_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
