#[doc = "Register `FDCAN_CREL` reader"]
pub struct R(crate::R<FDCAN_CREL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_CREL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_CREL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_CREL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAY` reader - 18"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` reader - 12"]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` reader - 4"]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBSTEP` reader - 1"]
pub type SUBSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEP` reader - 2"]
pub type STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` reader - 3"]
pub type REL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 18"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 12"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 4"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 1"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 2"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "FDCAN core release register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_crel](index.html) module"]
pub struct FDCAN_CREL_SPEC;
impl crate::RegisterSpec for FDCAN_CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_crel::R](R) reader structure"]
impl crate::Readable for FDCAN_CREL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_CREL to value 0x3214_1218"]
impl crate::Resettable for FDCAN_CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3214_1218;
}
