#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDPRT` reader - Read protection"]
pub type RDPRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` reader - BOR_LEV"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IWDG_SW` reader - IWDG_SW"]
pub type IWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `nRTS_STOP` reader - nRTS_STOP"]
pub type N_RTS_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `BFB2` reader - Boot From Bank 2"]
pub type BFB2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - IWDG_SW"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - nRTS_STOP"]
    #[inline(always)]
    pub fn n_rts_stop(&self) -> N_RTS_STOP_R {
        N_RTS_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot From Bank 2"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0x00f8_0000"]
impl crate::Resettable for OBR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f8_0000;
}
