#[doc = "Register `MACRxTxSR` reader"]
pub struct R(crate::R<MACRX_TX_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRX_TX_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRX_TX_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRX_TX_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub type TJT_R = crate::BitReader<bool>;
#[doc = "Field `NCARR` reader - No Carrier"]
pub type NCARR_R = crate::BitReader<bool>;
#[doc = "Field `LCARR` reader - Loss of Carrier"]
pub type LCARR_R = crate::BitReader<bool>;
#[doc = "Field `EXDEF` reader - Excessive Deferral"]
pub type EXDEF_R = crate::BitReader<bool>;
#[doc = "Field `LCOL` reader - Late Collision"]
pub type LCOL_R = crate::BitReader<bool>;
#[doc = "Field `EXCOL` reader - Excessive Collisions"]
pub type EXCOL_R = crate::BitReader<bool>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Carrier"]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Loss of Carrier"]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Excessive Deferral"]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Late Collision"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Rx Tx status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrx_tx_sr](index.html) module"]
pub struct MACRX_TX_SR_SPEC;
impl crate::RegisterSpec for MACRX_TX_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macrx_tx_sr::R](R) reader structure"]
impl crate::Readable for MACRX_TX_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACRxTxSR to value 0"]
impl crate::Resettable for MACRX_TX_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
