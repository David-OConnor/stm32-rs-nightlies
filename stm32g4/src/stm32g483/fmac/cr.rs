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
#[doc = "Field `RIEN` reader - RIEN"]
pub type RIEN_R = crate::BitReader<bool>;
#[doc = "Field `RIEN` writer - RIEN"]
pub type RIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WIEN` reader - WIEN"]
pub type WIEN_R = crate::BitReader<bool>;
#[doc = "Field `WIEN` writer - WIEN"]
pub type WIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OVFLIEN` reader - OVFLIEN"]
pub type OVFLIEN_R = crate::BitReader<bool>;
#[doc = "Field `OVFLIEN` writer - OVFLIEN"]
pub type OVFLIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `UNFLIEN` reader - UNFLIEN"]
pub type UNFLIEN_R = crate::BitReader<bool>;
#[doc = "Field `UNFLIEN` writer - UNFLIEN"]
pub type UNFLIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SATIEN` reader - SATIEN"]
pub type SATIEN_R = crate::BitReader<bool>;
#[doc = "Field `SATIEN` writer - SATIEN"]
pub type SATIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAREN` reader - DMAREN"]
pub type DMAREN_R = crate::BitReader<bool>;
#[doc = "Field `DMAREN` writer - DMAREN"]
pub type DMAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub type DMAWEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAWEN` writer - DMAWEN"]
pub type DMAWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CLIPEN` reader - CLIPEN"]
pub type CLIPEN_R = crate::BitReader<bool>;
#[doc = "Field `CLIPEN` writer - CLIPEN"]
pub type CLIPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RESET` reader - RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RIEN"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WIEN"]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OVFLIEN"]
    #[inline(always)]
    pub fn ovflien(&self) -> OVFLIEN_R {
        OVFLIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UNFLIEN"]
    #[inline(always)]
    pub fn unflien(&self) -> UNFLIEN_R {
        UNFLIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SATIEN"]
    #[inline(always)]
    pub fn satien(&self) -> SATIEN_R {
        SATIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - CLIPEN"]
    #[inline(always)]
    pub fn clipen(&self) -> CLIPEN_R {
        CLIPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<0> {
        RIEN_W::new(self)
    }
    #[doc = "Bit 1 - WIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wien(&mut self) -> WIEN_W<1> {
        WIEN_W::new(self)
    }
    #[doc = "Bit 2 - OVFLIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ovflien(&mut self) -> OVFLIEN_W<2> {
        OVFLIEN_W::new(self)
    }
    #[doc = "Bit 3 - UNFLIEN"]
    #[inline(always)]
    #[must_use]
    pub fn unflien(&mut self) -> UNFLIEN_W<3> {
        UNFLIEN_W::new(self)
    }
    #[doc = "Bit 4 - SATIEN"]
    #[inline(always)]
    #[must_use]
    pub fn satien(&mut self) -> SATIEN_W<4> {
        SATIEN_W::new(self)
    }
    #[doc = "Bit 8 - DMAREN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<8> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 9 - DMAWEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<9> {
        DMAWEN_W::new(self)
    }
    #[doc = "Bit 15 - CLIPEN"]
    #[inline(always)]
    #[must_use]
    pub fn clipen(&mut self) -> CLIPEN_W<15> {
        CLIPEN_W::new(self)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<16> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
