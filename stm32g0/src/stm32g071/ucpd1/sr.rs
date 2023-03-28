#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXIS` reader - TXIS"]
pub type TXIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGDISC` reader - TXMSGDISC"]
pub type TXMSGDISC_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGSENT` reader - TXMSGSENT"]
pub type TXMSGSENT_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGABT` reader - TXMSGABT"]
pub type TXMSGABT_R = crate::BitReader<bool>;
#[doc = "Field `HRSTDISC` reader - HRSTDISC"]
pub type HRSTDISC_R = crate::BitReader<bool>;
#[doc = "Field `HRSTSENT` reader - HRSTSENT"]
pub type HRSTSENT_R = crate::BitReader<bool>;
#[doc = "Field `TXUND` reader - TXUND"]
pub type TXUND_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` reader - RXNE"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `RXORDDET` reader - RXORDDET"]
pub type RXORDDET_R = crate::BitReader<bool>;
#[doc = "Field `RXHRSTDET` reader - RXHRSTDET"]
pub type RXHRSTDET_R = crate::BitReader<bool>;
#[doc = "Field `RXOVR` reader - RXOVR"]
pub type RXOVR_R = crate::BitReader<bool>;
#[doc = "Field `RXMSGEND` reader - RXMSGEND"]
pub type RXMSGEND_R = crate::BitReader<bool>;
#[doc = "Field `RXERR` reader - RXERR"]
pub type RXERR_R = crate::BitReader<bool>;
#[doc = "Field `TYPECEVT1` reader - TYPECEVT1"]
pub type TYPECEVT1_R = crate::BitReader<bool>;
#[doc = "Field `TYPECEVT2` reader - TYPECEVT2"]
pub type TYPECEVT2_R = crate::BitReader<bool>;
#[doc = "Field `TYPEC_VSTATE_CC1` reader - TYPEC_VSTATE_CC1"]
pub type TYPEC_VSTATE_CC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPEC_VSTATE_CC2` reader - TYPEC_VSTATE_CC2"]
pub type TYPEC_VSTATE_CC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRSEVT` reader - FRSEVT"]
pub type FRSEVT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "UCPD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
