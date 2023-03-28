#[doc = "Register `TAMP_CFGR` reader"]
pub struct R(crate::R<TAMP_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_CFGR` writer"]
pub struct W(crate::W<TAMP_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_CFGR_SPEC>;
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
impl From<crate::W<TAMP_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPRW` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKPRW` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_CFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKPW` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKPW` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
pub type BKPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_CFGR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkprw(&self) -> BKPRW_R {
        BKPRW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkpw(&self) -> BKPW_R {
        BKPW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkprw(&mut self) -> BKPRW_W<0> {
        BKPRW_W::new(self)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkpw(&mut self) -> BKPW_W<16> {
        BKPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_cfgr](index.html) module"]
pub struct TAMP_CFGR_SPEC;
impl crate::RegisterSpec for TAMP_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_cfgr::R](R) reader structure"]
impl crate::Readable for TAMP_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_cfgr::W](W) writer structure"]
impl crate::Writable for TAMP_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_CFGR to value 0"]
impl crate::Resettable for TAMP_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
