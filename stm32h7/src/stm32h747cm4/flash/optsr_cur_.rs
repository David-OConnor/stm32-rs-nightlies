#[doc = "Register `OPTSR_CUR_` reader"]
pub struct R(crate::R<OPTSR_CUR__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTSR_CUR__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTSR_CUR__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTSR_CUR__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTSR_CUR_` writer"]
pub struct W(crate::W<OPTSR_CUR__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTSR_CUR__SPEC>;
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
impl From<crate::W<OPTSR_CUR__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTSR_CUR__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPT_BUSY` reader - Option byte change ongoing flag"]
pub type OPT_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `OPT_BUSY` writer - Option byte change ongoing flag"]
pub type OPT_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `BOR_LEV` reader - Brownout level option status bit"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` writer - Brownout level option status bit"]
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTSR_CUR__SPEC, u8, u8, 2, O>;
#[doc = "Field `IWDG_SW` reader - IWDG control mode option status bit"]
pub type IWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_SW` writer - IWDG control mode option status bit"]
pub type IWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `IWDG2_SW` reader - IWDG2 control mode option status bit"]
pub type IWDG2_SW_R = crate::BitReader<bool>;
#[doc = "Field `IWDG2_SW` writer - IWDG2 control mode option status bit"]
pub type IWDG2_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `NRST_STOP_D1` reader - D1 domain DStop entry reset option status bit"]
pub type NRST_STOP_D1_R = crate::BitReader<bool>;
#[doc = "Field `NRST_STOP_D1` writer - D1 domain DStop entry reset option status bit"]
pub type NRST_STOP_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `RST_STDY_D1` reader - D1 domain DStandby entry reset option status bit"]
pub type RST_STDY_D1_R = crate::BitReader<bool>;
#[doc = "Field `RST_STDY_D1` writer - D1 domain DStandby entry reset option status bit"]
pub type RST_STDY_D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `RDP` reader - Readout protection level option status byte"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - Readout protection level option status byte"]
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTSR_CUR__SPEC, u8, u8, 8, O>;
#[doc = "Field `IWDG_FZ_STOP` reader - IWDG Stop mode freeze option status bit"]
pub type IWDG_FZ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_FZ_STOP` writer - IWDG Stop mode freeze option status bit"]
pub type IWDG_FZ_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `IWDG_FZ_SDBY` reader - IWDG Standby mode freeze option status bit"]
pub type IWDG_FZ_SDBY_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_FZ_SDBY` writer - IWDG Standby mode freeze option status bit"]
pub type IWDG_FZ_SDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `ST_RAM_SIZE` reader - ST RAM size option status"]
pub type ST_RAM_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST_RAM_SIZE` writer - ST RAM size option status"]
pub type ST_RAM_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPTSR_CUR__SPEC, u8, u8, 2, O>;
#[doc = "Field `SECURITY` reader - Security enable option status bit"]
pub type SECURITY_R = crate::BitReader<bool>;
#[doc = "Field `SECURITY` writer - Security enable option status bit"]
pub type SECURITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `BOOT_CM4` reader - Arm Cortex"]
pub type BOOT_CM4_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_CM4` writer - Arm Cortex"]
pub type BOOT_CM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `BOOT_CM7` reader - Arm Cortex"]
pub type BOOT_CM7_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_CM7` writer - Arm Cortex"]
pub type BOOT_CM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `NRST_STOP_D2` reader - D2 domain DStop entry reset option status bit"]
pub type NRST_STOP_D2_R = crate::BitReader<bool>;
#[doc = "Field `NRST_STOP_D2` writer - D2 domain DStop entry reset option status bit"]
pub type NRST_STOP_D2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `NRST_STBY_D2` reader - D2 domain DStandby entry reset option status bit"]
pub type NRST_STBY_D2_R = crate::BitReader<bool>;
#[doc = "Field `NRST_STBY_D2` writer - D2 domain DStandby entry reset option status bit"]
pub type NRST_STBY_D2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `IO_HSLV` reader - I"]
pub type IO_HSLV_R = crate::BitReader<bool>;
#[doc = "Field `IO_HSLV` writer - I"]
pub type IO_HSLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag"]
pub type OPTCHANGEERR_R = crate::BitReader<bool>;
#[doc = "Field `OPTCHANGEERR` writer - Option byte change error flag"]
pub type OPTCHANGEERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option status bit"]
pub type SWAP_BANK_OPT_R = crate::BitReader<bool>;
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option status bit"]
pub type SWAP_BANK_OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR_CUR__SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&self) -> OPT_BUSY_R {
        OPT_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - IWDG control mode option status bit"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IWDG2 control mode option status bit"]
    #[inline(always)]
    pub fn iwdg2_sw(&self) -> IWDG2_SW_R {
        IWDG2_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn rst_stdy_d1(&self) -> RST_STDY_D1_R {
        RST_STDY_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - ST RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm4(&self) -> BOOT_CM4_R {
        BOOT_CM4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm7(&self) -> BOOT_CM7_R {
        BOOT_CM7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stop_d2(&self) -> NRST_STOP_D2_R {
        NRST_STOP_D2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn nrst_stby_d2(&self) -> NRST_STBY_D2_R {
        NRST_STBY_D2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    #[must_use]
    pub fn opt_busy(&mut self) -> OPT_BUSY_W<0> {
        OPT_BUSY_W::new(self)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<2> {
        BOR_LEV_W::new(self)
    }
    #[doc = "Bit 4 - IWDG control mode option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<4> {
        IWDG_SW_W::new(self)
    }
    #[doc = "Bit 5 - IWDG2 control mode option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg2_sw(&mut self) -> IWDG2_SW_W<5> {
        IWDG2_SW_W::new(self)
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stop_d1(&mut self) -> NRST_STOP_D1_W<6> {
        NRST_STOP_D1_W::new(self)
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst_stdy_d1(&mut self) -> RST_STDY_D1_W<7> {
        RST_STDY_D1_W::new(self)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<8> {
        RDP_W::new(self)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_fz_stop(&mut self) -> IWDG_FZ_STOP_W<17> {
        IWDG_FZ_STOP_W::new(self)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W<18> {
        IWDG_FZ_SDBY_W::new(self)
    }
    #[doc = "Bits 19:20 - ST RAM size option status"]
    #[inline(always)]
    #[must_use]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W<19> {
        ST_RAM_SIZE_W::new(self)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<21> {
        SECURITY_W::new(self)
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm4(&mut self) -> BOOT_CM4_W<22> {
        BOOT_CM4_W::new(self)
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm7(&mut self) -> BOOT_CM7_W<23> {
        BOOT_CM7_W::new(self)
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stop_d2(&mut self) -> NRST_STOP_D2_W<24> {
        NRST_STOP_D2_W::new(self)
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stby_d2(&mut self) -> NRST_STBY_D2_W<25> {
        NRST_STBY_D2_W::new(self)
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    #[must_use]
    pub fn io_hslv(&mut self) -> IO_HSLV_W<29> {
        IO_HSLV_W::new(self)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    #[must_use]
    pub fn optchangeerr(&mut self) -> OPTCHANGEERR_W<30> {
        OPTCHANGEERR_W::new(self)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W<31> {
        SWAP_BANK_OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr_cur_](index.html) module"]
pub struct OPTSR_CUR__SPEC;
impl crate::RegisterSpec for OPTSR_CUR__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optsr_cur_::R](R) reader structure"]
impl crate::Readable for OPTSR_CUR__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optsr_cur_::W](W) writer structure"]
impl crate::Writable for OPTSR_CUR__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTSR_CUR_ to value 0"]
impl crate::Resettable for OPTSR_CUR__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
