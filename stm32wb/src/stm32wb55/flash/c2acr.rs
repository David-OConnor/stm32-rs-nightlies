#[doc = "Register `C2ACR` reader"]
pub struct R(crate::R<C2ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2ACR` writer"]
pub struct W(crate::W<C2ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2ACR_SPEC>;
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
impl From<crate::W<C2ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRFTEN` reader - CPU2 cortex M0 prefetch enable"]
pub type PRFTEN_R = crate::BitReader<bool>;
#[doc = "Field `PRFTEN` writer - CPU2 cortex M0 prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, bool, O>;
#[doc = "Field `ICEN` reader - CPU2 cortex M0 instruction cache enable"]
pub type ICEN_R = crate::BitReader<bool>;
#[doc = "Field `ICEN` writer - CPU2 cortex M0 instruction cache enable"]
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, bool, O>;
#[doc = "Field `ICRST` reader - CPU2 cortex M0 instruction cache reset"]
pub type ICRST_R = crate::BitReader<bool>;
#[doc = "Field `ICRST` writer - CPU2 cortex M0 instruction cache reset"]
pub type ICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, bool, O>;
#[doc = "Field `PES` reader - CPU2 cortex M0 program erase suspend request"]
pub type PES_R = crate::BitReader<bool>;
#[doc = "Field `PES` writer - CPU2 cortex M0 program erase suspend request"]
pub type PES_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - CPU2 cortex M0 prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 cortex M0 instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU2 cortex M0 instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 cortex M0 program erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CPU2 cortex M0 prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    #[doc = "Bit 9 - CPU2 cortex M0 instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<9> {
        ICEN_W::new(self)
    }
    #[doc = "Bit 11 - CPU2 cortex M0 instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<11> {
        ICRST_W::new(self)
    }
    #[doc = "Bit 15 - CPU2 cortex M0 program erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<15> {
        PES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 cortex M0 access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2acr](index.html) module"]
pub struct C2ACR_SPEC;
impl crate::RegisterSpec for C2ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2acr::R](R) reader structure"]
impl crate::Readable for C2ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2acr::W](W) writer structure"]
impl crate::Writable for C2ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2ACR to value 0x0600"]
impl crate::Resettable for C2ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
