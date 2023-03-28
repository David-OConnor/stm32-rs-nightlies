#[doc = "Register `EXTCFGR` reader"]
pub struct R(crate::R<EXTCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTCFGR` writer"]
pub struct W(crate::W<EXTCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCFGR_SPEC>;
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
impl From<crate::W<EXTCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDHPRE` reader - Shared AHB prescaler"]
pub type SHDHPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHDHPRE` writer - Shared AHB prescaler"]
pub type SHDHPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTCFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `C2HPRE` reader - CPU2 AHB prescaler"]
pub type C2HPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `C2HPRE` writer - CPU2 AHB prescaler"]
pub type C2HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTCFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SHDHPREF` reader - Shared AHB prescaler flag"]
pub type SHDHPREF_R = crate::BitReader<bool>;
#[doc = "Field `C2HPREF` reader - CPU2 AHB prescaler flag"]
pub type C2HPREF_R = crate::BitReader<bool>;
#[doc = "Field `RFCSS` reader - RF clock source selected"]
pub type RFCSS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Shared AHB prescaler flag"]
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 AHB prescaler flag"]
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - RF clock source selected"]
    #[inline(always)]
    pub fn rfcss(&self) -> RFCSS_R {
        RFCSS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shared AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<0> {
        SHDHPRE_W::new(self)
    }
    #[doc = "Bits 4:7 - CPU2 AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn c2hpre(&mut self) -> C2HPRE_W<4> {
        C2HPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended clock recovery register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extcfgr](index.html) module"]
pub struct EXTCFGR_SPEC;
impl crate::RegisterSpec for EXTCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extcfgr::R](R) reader structure"]
impl crate::Readable for EXTCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extcfgr::W](W) writer structure"]
impl crate::Writable for EXTCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTCFGR to value 0x0003_0000"]
impl crate::Resettable for EXTCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0000;
}
