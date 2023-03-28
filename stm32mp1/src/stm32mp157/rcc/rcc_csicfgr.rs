#[doc = "Register `RCC_CSICFGR` reader"]
pub struct R(crate::R<RCC_CSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CSICFGR` writer"]
pub struct W(crate::W<RCC_CSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CSICFGR_SPEC>;
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
impl From<crate::W<RCC_CSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSITRIM` reader - CSITRIM"]
pub type CSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSITRIM` writer - CSITRIM"]
pub type CSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CSICFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CSICAL` reader - CSICAL"]
pub type CSICAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:12 - CSITRIM"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - CSICAL"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - CSITRIM"]
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<8> {
        CSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_csicfgr](index.html) module"]
pub struct RCC_CSICFGR_SPEC;
impl crate::RegisterSpec for RCC_CSICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_csicfgr::R](R) reader structure"]
impl crate::Readable for RCC_CSICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_csicfgr::W](W) writer structure"]
impl crate::Writable for RCC_CSICFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CSICFGR to value 0x1000"]
impl crate::Resettable for RCC_CSICFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
