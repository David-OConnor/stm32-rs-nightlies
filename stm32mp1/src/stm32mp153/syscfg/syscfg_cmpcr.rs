#[doc = "Register `SYSCFG_CMPCR` reader"]
pub struct R(crate::R<SYSCFG_CMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CMPCR` writer"]
pub struct W(crate::W<SYSCFG_CMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CMPCR_SPEC>;
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
impl From<crate::W<SYSCFG_CMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_CTRL` reader - SW_CTRL"]
pub type SW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `SW_CTRL` writer - SW_CTRL"]
pub type SW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CMPCR_SPEC, bool, O>;
#[doc = "Field `READY` reader - READY"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `RANSRC` reader - RANSRC"]
pub type RANSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANSRC` writer - RANSRC"]
pub type RANSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CMPCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RAPSRC` reader - RAPSRC"]
pub type RAPSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAPSRC` writer - RAPSRC"]
pub type RAPSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCFG_CMPCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ANSRC` reader - ANSRC"]
pub type ANSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APSRC` reader - APSRC"]
pub type APSRC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    pub fn sw_ctrl(&self) -> SW_CTRL_R {
        SW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    pub fn ransrc(&self) -> RANSRC_R {
        RANSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    pub fn rapsrc(&self) -> RAPSRC_R {
        RAPSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ANSRC"]
    #[inline(always)]
    pub fn ansrc(&self) -> ANSRC_R {
        ANSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - APSRC"]
    #[inline(always)]
    pub fn apsrc(&self) -> APSRC_R {
        APSRC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ctrl(&mut self) -> SW_CTRL_W<1> {
        SW_CTRL_W::new(self)
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ransrc(&mut self) -> RANSRC_W<16> {
        RANSRC_W::new(self)
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    #[must_use]
    pub fn rapsrc(&mut self) -> RAPSRC_W<20> {
        RAPSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpcr](index.html) module"]
pub struct SYSCFG_CMPCR_SPEC;
impl crate::RegisterSpec for SYSCFG_CMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cmpcr::R](R) reader structure"]
impl crate::Readable for SYSCFG_CMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpcr::W](W) writer structure"]
impl crate::Writable for SYSCFG_CMPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CMPCR to value 0x0087_0000"]
impl crate::Resettable for SYSCFG_CMPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0087_0000;
}
