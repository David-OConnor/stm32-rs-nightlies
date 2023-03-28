#[doc = "Register `HASH_IMR` reader"]
pub struct R(crate::R<HASH_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_IMR` writer"]
pub struct W(crate::W<HASH_IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_IMR_SPEC>;
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
impl From<crate::W<HASH_IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DINIE` reader - DINIE"]
pub type DINIE_R = crate::BitReader<bool>;
#[doc = "Field `DINIE` writer - DINIE"]
pub type DINIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_IMR_SPEC, bool, O>;
#[doc = "Field `DCIE` reader - DCIE"]
pub type DCIE_R = crate::BitReader<bool>;
#[doc = "Field `DCIE` writer - DCIE"]
pub type DCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DINIE"]
    #[inline(always)]
    pub fn dinie(&self) -> DINIE_R {
        DINIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCIE"]
    #[inline(always)]
    pub fn dcie(&self) -> DCIE_R {
        DCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DINIE"]
    #[inline(always)]
    #[must_use]
    pub fn dinie(&mut self) -> DINIE_W<0> {
        DINIE_W::new(self)
    }
    #[doc = "Bit 1 - DCIE"]
    #[inline(always)]
    #[must_use]
    pub fn dcie(&mut self) -> DCIE_W<1> {
        DCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_imr](index.html) module"]
pub struct HASH_IMR_SPEC;
impl crate::RegisterSpec for HASH_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_imr::R](R) reader structure"]
impl crate::Readable for HASH_IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_imr::W](W) writer structure"]
impl crate::Writable for HASH_IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_IMR to value 0"]
impl crate::Resettable for HASH_IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
