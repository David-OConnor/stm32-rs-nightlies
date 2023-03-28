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
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OPTERR_R = crate::BitReader<bool>;
#[doc = "Field `RDPRT` reader - Read protection"]
pub type RDPRT_R = crate::BitReader<bool>;
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `Data0` reader - Data0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data1` reader - Data1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 10:17 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 18) & 0xff) as u8)
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
#[doc = "`reset()` method sets OBR to value 0x03ff_fffc"]
impl crate::Resettable for OBR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_fffc;
}
