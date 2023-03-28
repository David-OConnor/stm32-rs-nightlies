#[doc = "Register `FCCAN_CCU_CREL` reader"]
pub struct R(crate::R<FCCAN_CCU_CREL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_CREL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_CREL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_CREL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAY` reader - DAY"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` reader - MON"]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` reader - YEAR"]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBSTEP` reader - SUBSTEP"]
pub type SUBSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEP` reader - STEP"]
pub type STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` reader - REL"]
pub type REL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DAY"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MON"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - YEAR"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SUBSTEP"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - STEP"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - REL"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Clock calibration unit core release register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_crel](index.html) module"]
pub struct FCCAN_CCU_CREL_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_crel::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_CREL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCCAN_CCU_CREL to value 0x1114_1218"]
impl crate::Resettable for FCCAN_CCU_CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1114_1218;
}
