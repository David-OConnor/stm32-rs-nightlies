#[doc = "Register `RCC_HSICFGR` reader"]
pub struct R(crate::R<RCC_HSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_HSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_HSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_HSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_HSICFGR` writer"]
pub struct W(crate::W<RCC_HSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_HSICFGR_SPEC>;
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
impl From<crate::W<RCC_HSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_HSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSIDIV` reader - HSIDIV"]
pub type HSIDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSIDIV` writer - HSIDIV"]
pub type HSIDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_HSICFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `HSITRIM` reader - HSITRIM"]
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` writer - HSITRIM"]
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_HSICFGR_SPEC, u8, u8, 7, O>;
#[doc = "Field `HSICAL` reader - HSICAL"]
pub type HSICAL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<0> {
        HSIDIV_W::new(self)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<8> {
        HSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_hsicfgr](index.html) module"]
pub struct RCC_HSICFGR_SPEC;
impl crate::RegisterSpec for RCC_HSICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_hsicfgr::R](R) reader structure"]
impl crate::Readable for RCC_HSICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_hsicfgr::W](W) writer structure"]
impl crate::Writable for RCC_HSICFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_HSICFGR to value 0"]
impl crate::Resettable for RCC_HSICFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
