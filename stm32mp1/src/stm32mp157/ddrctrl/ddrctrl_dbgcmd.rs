#[doc = "Register `DDRCTRL_DBGCMD` reader"]
pub struct R(crate::R<DDRCTRL_DBGCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DBGCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DBGCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DBGCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DBGCMD` writer"]
pub struct W(crate::W<DDRCTRL_DBGCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DBGCMD_SPEC>;
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
impl From<crate::W<DDRCTRL_DBGCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DBGCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANK0_REFRESH` reader - RANK0_REFRESH"]
pub type RANK0_REFRESH_R = crate::BitReader<bool>;
#[doc = "Field `RANK0_REFRESH` writer - RANK0_REFRESH"]
pub type RANK0_REFRESH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_DBGCMD_SPEC, bool, O>;
#[doc = "Field `ZQ_CALIB_SHORT` reader - ZQ_CALIB_SHORT"]
pub type ZQ_CALIB_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `ZQ_CALIB_SHORT` writer - ZQ_CALIB_SHORT"]
pub type ZQ_CALIB_SHORT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DBGCMD_SPEC, bool, O>;
#[doc = "Field `CTRLUPD` reader - CTRLUPD"]
pub type CTRLUPD_R = crate::BitReader<bool>;
#[doc = "Field `CTRLUPD` writer - CTRLUPD"]
pub type CTRLUPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_DBGCMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RANK0_REFRESH"]
    #[inline(always)]
    pub fn rank0_refresh(&self) -> RANK0_REFRESH_R {
        RANK0_REFRESH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT"]
    #[inline(always)]
    pub fn zq_calib_short(&self) -> ZQ_CALIB_SHORT_R {
        ZQ_CALIB_SHORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTRLUPD"]
    #[inline(always)]
    pub fn ctrlupd(&self) -> CTRLUPD_R {
        CTRLUPD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RANK0_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn rank0_refresh(&mut self) -> RANK0_REFRESH_W<0> {
        RANK0_REFRESH_W::new(self)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT"]
    #[inline(always)]
    #[must_use]
    pub fn zq_calib_short(&mut self) -> ZQ_CALIB_SHORT_W<4> {
        ZQ_CALIB_SHORT_W::new(self)
    }
    #[doc = "Bit 5 - CTRLUPD"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd(&mut self) -> CTRLUPD_W<5> {
        CTRLUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL command debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbgcmd](index.html) module"]
pub struct DDRCTRL_DBGCMD_SPEC;
impl crate::RegisterSpec for DDRCTRL_DBGCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dbgcmd::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DBGCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dbgcmd::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DBGCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DBGCMD to value 0"]
impl crate::Resettable for DDRCTRL_DBGCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
