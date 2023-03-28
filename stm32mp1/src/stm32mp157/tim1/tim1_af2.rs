#[doc = "Register `TIM1_AF2` reader"]
pub struct R(crate::R<TIM1_AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_AF2` writer"]
pub struct W(crate::W<TIM1_AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_AF2_SPEC>;
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
impl From<crate::W<TIM1_AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BK2INE` reader - BK2INE"]
pub type BK2INE_R = crate::BitReader<bool>;
#[doc = "Field `BK2INE` writer - BK2INE"]
pub type BK2INE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
#[doc = "Field `BK2DF1BK1E` reader - BK2DF1BK1E"]
pub type BK2DF1BK1E_R = crate::BitReader<bool>;
#[doc = "Field `BK2DF1BK1E` writer - BK2DF1BK1E"]
pub type BK2DF1BK1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
#[doc = "Field `BK2INP` reader - BK2INP"]
pub type BK2INP_R = crate::BitReader<bool>;
#[doc = "Field `BK2INP` writer - BK2INP"]
pub type BK2INP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_AF2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - BK2DF1BK1E"]
    #[inline(always)]
    pub fn bk2df1bk1e(&self) -> BK2DF1BK1E_R {
        BK2DF1BK1E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<0> {
        BK2INE_W::new(self)
    }
    #[doc = "Bit 8 - BK2DF1BK1E"]
    #[inline(always)]
    #[must_use]
    pub fn bk2df1bk1e(&mut self) -> BK2DF1BK1E_W<8> {
        BK2DF1BK1E_W::new(self)
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<9> {
        BK2INP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 Alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af2](index.html) module"]
pub struct TIM1_AF2_SPEC;
impl crate::RegisterSpec for TIM1_AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_af2::R](R) reader structure"]
impl crate::Readable for TIM1_AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_af2::W](W) writer structure"]
impl crate::Writable for TIM1_AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1_AF2 to value 0x01"]
impl crate::Resettable for TIM1_AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
