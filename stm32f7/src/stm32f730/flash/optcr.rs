#[doc = "Register `OPTCR` reader"]
pub struct R(crate::R<OPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR` writer"]
pub struct W(crate::W<OPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR_SPEC>;
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
impl From<crate::W<OPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTLOCK` reader - Option lock"]
pub type OPTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OPTLOCK` writer - Option lock"]
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `OPTSTRT` reader - Option start"]
pub type OPTSTRT_R = crate::BitReader<bool>;
#[doc = "Field `OPTSTRT` writer - Option start"]
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WWDG_SW` reader - User option bytes"]
pub type WWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `WWDG_SW` writer - User option bytes"]
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `IWDG_SW` reader - WDG_SW User option bytes"]
pub type IWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_SW` writer - WDG_SW User option bytes"]
pub type IWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP User option bytes"]
pub type N_RST_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP` writer - nRST_STOP User option bytes"]
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY User option bytes"]
pub type N_RST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY User option bytes"]
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `RDP` reader - Read protect"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - Read protect"]
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `nWRP` reader - Not write protect"]
pub type N_WRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `nWRP` writer - Not write protect"]
pub type N_WRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in standby mode"]
pub type IWDG_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in standby mode"]
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Option lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - User option bytes"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDG_SW User option bytes"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - nRST_STOP User option bytes"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - nRST_STDBY User option bytes"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Read protect"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Independent watchdog counter freeze in standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<0> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Option start"]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<1> {
        OPTSTRT_W::new(self)
    }
    #[doc = "Bits 2:3 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<2> {
        BOR_LEV_W::new(self)
    }
    #[doc = "Bit 4 - User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<4> {
        WWDG_SW_W::new(self)
    }
    #[doc = "Bit 5 - WDG_SW User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<5> {
        IWDG_SW_W::new(self)
    }
    #[doc = "Bit 6 - nRST_STOP User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<6> {
        N_RST_STOP_W::new(self)
    }
    #[doc = "Bit 7 - nRST_STDBY User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<7> {
        N_RST_STDBY_W::new(self)
    }
    #[doc = "Bits 8:15 - Read protect"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<8> {
        RDP_W::new(self)
    }
    #[doc = "Bits 16:23 - Not write protect"]
    #[inline(always)]
    #[must_use]
    pub fn n_wrp(&mut self) -> N_WRP_W<16> {
        N_WRP_W::new(self)
    }
    #[doc = "Bit 30 - Independent watchdog counter freeze in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<30> {
        IWDG_STDBY_W::new(self)
    }
    #[doc = "Bit 31 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<31> {
        IWDG_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr](index.html) module"]
pub struct OPTCR_SPEC;
impl crate::RegisterSpec for OPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr::R](R) reader structure"]
impl crate::Readable for OPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr::W](W) writer structure"]
impl crate::Writable for OPTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTCR to value 0xc0ff_aafd"]
impl crate::Resettable for OPTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0ff_aafd;
}
