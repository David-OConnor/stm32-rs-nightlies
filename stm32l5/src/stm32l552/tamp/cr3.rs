#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITAMP1NOER` reader - ITAMP1NOER"]
pub type ITAMP1NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP1NOER` writer - ITAMP1NOER"]
pub type ITAMP1NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP2NOER` reader - ITAMP2NOER"]
pub type ITAMP2NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP2NOER` writer - ITAMP2NOER"]
pub type ITAMP2NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP3NOER` reader - ITAMP3NOER"]
pub type ITAMP3NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP3NOER` writer - ITAMP3NOER"]
pub type ITAMP3NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP5NOER` reader - ITAMP5NOER"]
pub type ITAMP5NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP5NOER` writer - ITAMP5NOER"]
pub type ITAMP5NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP8NOER` reader - ITAMP8NOER"]
pub type ITAMP8NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP8NOER` writer - ITAMP8NOER"]
pub type ITAMP8NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<0> {
        ITAMP1NOER_W::new(self)
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<1> {
        ITAMP2NOER_W::new(self)
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<2> {
        ITAMP3NOER_W::new(self)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<4> {
        ITAMP5NOER_W::new(self)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<7> {
        ITAMP8NOER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
