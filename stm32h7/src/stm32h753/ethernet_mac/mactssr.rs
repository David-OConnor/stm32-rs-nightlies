#[doc = "Register `MACTSSR` reader"]
pub struct R(crate::R<MACTSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow"]
pub type TSSOVF_R = crate::BitReader<bool>;
#[doc = "Field `TSTARGT0` reader - Timestamp Target Time Reached"]
pub type TSTARGT0_R = crate::BitReader<bool>;
#[doc = "Field `AUXTSTRIG` reader - Auxiliary Timestamp Trigger Snapshot"]
pub type AUXTSTRIG_R = crate::BitReader<bool>;
#[doc = "Field `TSTRGTERR0` reader - Timestamp Target Time Error"]
pub type TSTRGTERR0_R = crate::BitReader<bool>;
#[doc = "Field `TXTSSIS` reader - Tx Timestamp Status Interrupt Status"]
pub type TXTSSIS_R = crate::BitReader<bool>;
#[doc = "Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier"]
pub type ATSSTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed"]
pub type ATSSTM_R = crate::BitReader<bool>;
#[doc = "Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots"]
pub type ATSNS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Timestamp Status Interrupt Status"]
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier"]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Number of Auxiliary Timestamp Snapshots"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[doc = "Timestamp status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactssr](index.html) module"]
pub struct MACTSSR_SPEC;
impl crate::RegisterSpec for MACTSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactssr::R](R) reader structure"]
impl crate::Readable for MACTSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACTSSR to value 0"]
impl crate::Resettable for MACTSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
