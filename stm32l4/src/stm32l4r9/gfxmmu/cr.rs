#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0OIE` reader - Buffer 0 overflow interrupt enable"]
pub type B0OIE_R = crate::BitReader<bool>;
#[doc = "Field `B0OIE` writer - Buffer 0 overflow interrupt enable"]
pub type B0OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `B1OIE` reader - Buffer 1 overflow interrupt enable"]
pub type B1OIE_R = crate::BitReader<bool>;
#[doc = "Field `B1OIE` writer - Buffer 1 overflow interrupt enable"]
pub type B1OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `B2OIE` reader - Buffer 2 overflow interrupt enable"]
pub type B2OIE_R = crate::BitReader<bool>;
#[doc = "Field `B2OIE` writer - Buffer 2 overflow interrupt enable"]
pub type B2OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `B3OIE` reader - Buffer 3 overflow interrupt enable"]
pub type B3OIE_R = crate::BitReader<bool>;
#[doc = "Field `B3OIE` writer - Buffer 3 overflow interrupt enable"]
pub type B3OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AMEIE` reader - AHB master error interrupt enable"]
pub type AMEIE_R = crate::BitReader<bool>;
#[doc = "Field `AMEIE` writer - AHB master error interrupt enable"]
pub type AMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BM192` reader - 192 Block mode"]
pub type BM192_R = crate::BitReader<bool>;
#[doc = "Field `BM192` writer - 192 Block mode"]
pub type BM192_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Buffer 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer 2 overflow interrupt enable"]
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer 3 overflow interrupt enable"]
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable"]
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - 192 Block mode"]
    #[inline(always)]
    pub fn bm192(&self) -> BM192_R {
        BM192_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer 0 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0oie(&mut self) -> B0OIE_W<0> {
        B0OIE_W::new(self)
    }
    #[doc = "Bit 1 - Buffer 1 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1oie(&mut self) -> B1OIE_W<1> {
        B1OIE_W::new(self)
    }
    #[doc = "Bit 2 - Buffer 2 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b2oie(&mut self) -> B2OIE_W<2> {
        B2OIE_W::new(self)
    }
    #[doc = "Bit 3 - Buffer 3 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn b3oie(&mut self) -> B3OIE_W<3> {
        B3OIE_W::new(self)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ameie(&mut self) -> AMEIE_W<4> {
        AMEIE_W::new(self)
    }
    #[doc = "Bit 6 - 192 Block mode"]
    #[inline(always)]
    #[must_use]
    pub fn bm192(&mut self) -> BM192_W<6> {
        BM192_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
