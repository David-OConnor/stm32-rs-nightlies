#[doc = "Register `FTSR2` reader"]
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR2` writer"]
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT35` reader - FT35"]
pub type FT35_R = crate::BitReader<bool>;
#[doc = "Field `FT35` writer - FT35"]
pub type FT35_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
#[doc = "Field `FT36` reader - FT36"]
pub type FT36_R = crate::BitReader<bool>;
#[doc = "Field `FT36` writer - FT36"]
pub type FT36_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
#[doc = "Field `FT37` reader - FT37"]
pub type FT37_R = crate::BitReader<bool>;
#[doc = "Field `FT37` writer - FT37"]
pub type FT37_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
#[doc = "Field `FT38` reader - FT38"]
pub type FT38_R = crate::BitReader<bool>;
#[doc = "Field `FT38` writer - FT38"]
pub type FT38_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - FT35"]
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FT36"]
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FT37"]
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FT38"]
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FT35"]
    #[inline(always)]
    #[must_use]
    pub fn ft35(&mut self) -> FT35_W<3> {
        FT35_W::new(self)
    }
    #[doc = "Bit 4 - FT36"]
    #[inline(always)]
    #[must_use]
    pub fn ft36(&mut self) -> FT36_W<4> {
        FT36_W::new(self)
    }
    #[doc = "Bit 5 - FT37"]
    #[inline(always)]
    #[must_use]
    pub fn ft37(&mut self) -> FT37_W<5> {
        FT37_W::new(self)
    }
    #[doc = "Bit 6 - FT38"]
    #[inline(always)]
    #[must_use]
    pub fn ft38(&mut self) -> FT38_W<6> {
        FT38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](index.html) module"]
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr2::R](R) reader structure"]
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](W) writer structure"]
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
