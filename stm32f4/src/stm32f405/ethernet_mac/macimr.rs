#[doc = "Register `MACIMR` reader"]
pub struct R(crate::R<MACIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACIMR` writer"]
pub struct W(crate::W<MACIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIMR_SPEC>;
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
impl From<crate::W<MACIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMTIM` reader - PMTIM"]
pub type PMTIM_R = crate::BitReader<bool>;
#[doc = "Field `PMTIM` writer - PMTIM"]
pub type PMTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIMR_SPEC, bool, O>;
#[doc = "Field `TSTIM` reader - TSTIM"]
pub type TSTIM_R = crate::BitReader<bool>;
#[doc = "Field `TSTIM` writer - TSTIM"]
pub type TSTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - PMTIM"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - TSTIM"]
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMTIM"]
    #[inline(always)]
    #[must_use]
    pub fn pmtim(&mut self) -> PMTIM_W<3> {
        PMTIM_W::new(self)
    }
    #[doc = "Bit 9 - TSTIM"]
    #[inline(always)]
    #[must_use]
    pub fn tstim(&mut self) -> TSTIM_W<9> {
        TSTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macimr](index.html) module"]
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macimr::R](R) reader structure"]
impl crate::Readable for MACIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macimr::W](W) writer structure"]
impl crate::Writable for MACIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}