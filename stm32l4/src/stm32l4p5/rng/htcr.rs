#[doc = "Register `HTCR` reader"]
pub struct R(crate::R<HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTCR` writer"]
pub struct W(crate::W<HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTCR_SPEC>;
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
impl From<crate::W<HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTCFG` reader - health test configuration"]
pub type HTCFG_R = crate::FieldReader<u32, HTCFG_A>;
#[doc = "health test configuration\n\nValue on reset: 23118"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum HTCFG_A {
    #[doc = "43636: Recommended value for RNG certification (0x0000_AA74)"]
    Recommended = 43636,
    #[doc = "391711420: Magic number to be written before any write (0x1759_0ABC)"]
    Magic = 391711420,
}
impl From<HTCFG_A> for u32 {
    #[inline(always)]
    fn from(variant: HTCFG_A) -> Self {
        variant as _
    }
}
impl HTCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HTCFG_A> {
        match self.bits {
            43636 => Some(HTCFG_A::Recommended),
            391711420 => Some(HTCFG_A::Magic),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Recommended`"]
    #[inline(always)]
    pub fn is_recommended(&self) -> bool {
        *self == HTCFG_A::Recommended
    }
    #[doc = "Checks if the value of the field is `Magic`"]
    #[inline(always)]
    pub fn is_magic(&self) -> bool {
        *self == HTCFG_A::Magic
    }
}
#[doc = "Field `HTCFG` writer - health test configuration"]
pub type HTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTCR_SPEC, u32, HTCFG_A, 32, O>;
impl<'a, const O: u8> HTCFG_W<'a, O> {
    #[doc = "Recommended value for RNG certification (0x0000_AA74)"]
    #[inline(always)]
    pub fn recommended(self) -> &'a mut W {
        self.variant(HTCFG_A::Recommended)
    }
    #[doc = "Magic number to be written before any write (0x1759_0ABC)"]
    #[inline(always)]
    pub fn magic(self) -> &'a mut W {
        self.variant(HTCFG_A::Magic)
    }
}
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    #[must_use]
    pub fn htcfg(&mut self) -> HTCFG_W<0> {
        HTCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "health test control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcr](index.html) module"]
pub struct HTCR_SPEC;
impl crate::RegisterSpec for HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcr::R](R) reader structure"]
impl crate::Readable for HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htcr::W](W) writer structure"]
impl crate::Writable for HTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTCR to value 0x5a4e"]
impl crate::Resettable for HTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x5a4e;
}
