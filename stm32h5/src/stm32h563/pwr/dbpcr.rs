#[doc = "Register `DBPCR` reader"]
pub struct R(crate::R<DBPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBPCR` writer"]
pub struct W(crate::W<DBPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBPCR_SPEC>;
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
impl From<crate::W<DBPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBP` reader - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_R = crate::BitReader<bool>;
#[doc = "Field `DBP` writer - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<0> {
        DBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbpcr](index.html) module"]
pub struct DBPCR_SPEC;
impl crate::RegisterSpec for DBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbpcr::R](R) reader structure"]
impl crate::Readable for DBPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbpcr::W](W) writer structure"]
impl crate::Writable for DBPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBPCR to value 0"]
impl crate::Resettable for DBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
