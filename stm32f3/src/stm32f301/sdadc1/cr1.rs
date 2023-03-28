#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOCALIE` reader - End of calibration interrupt enable"]
pub type EOCALIE_R = crate::BitReader<bool>;
#[doc = "Field `EOCALIE` writer - End of calibration interrupt enable"]
pub type EOCALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub type JEOCIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub type JOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub type JOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub type REOCIE_R = crate::BitReader<bool>;
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub type REOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub type ROVRIE_R = crate::BitReader<bool>;
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub type ROVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `REFV` reader - Reference voltage selection"]
pub type REFV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFV` writer - Reference voltage selection"]
pub type REFV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SLOWCK` reader - Slow clock mode enable"]
pub type SLOWCK_R = crate::BitReader<bool>;
#[doc = "Field `SLOWCK` writer - Slow clock mode enable"]
pub type SLOWCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SBI` reader - Enter Standby mode when idle"]
pub type SBI_R = crate::BitReader<bool>;
#[doc = "Field `SBI` writer - Enter Standby mode when idle"]
pub type SBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PDI` reader - Enter power down mode when idle"]
pub type PDI_R = crate::BitReader<bool>;
#[doc = "Field `PDI` writer - Enter power down mode when idle"]
pub type PDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JSYNC` reader - Launch a injected conversion synchronously with SDADC1"]
pub type JSYNC_R = crate::BitReader<bool>;
#[doc = "Field `JSYNC` writer - Launch a injected conversion synchronously with SDADC1"]
pub type JSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with SDADC1"]
pub type RSYNC_R = crate::BitReader<bool>;
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with SDADC1"]
pub type RSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group"]
pub type JDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular channel"]
pub type RDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular channel"]
pub type RDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `INIT` reader - Initialization mode request"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialization mode request"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - End of calibration interrupt enable"]
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Reference voltage selection"]
    #[inline(always)]
    pub fn refv(&self) -> REFV_R {
        REFV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Slow clock mode enable"]
    #[inline(always)]
    pub fn slowck(&self) -> SLOWCK_R {
        SLOWCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enter Standby mode when idle"]
    #[inline(always)]
    pub fn sbi(&self) -> SBI_R {
        SBI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enter power down mode when idle"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Launch a injected conversion synchronously with SDADC1"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Launch regular conversion synchronously with SDADC1"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA channel enabled to read data for the regular channel"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Initialization mode request"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of calibration interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocalie(&mut self) -> EOCALIE_W<0> {
        EOCALIE_W::new(self)
    }
    #[doc = "Bit 1 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<1> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jovrie(&mut self) -> JOVRIE_W<2> {
        JOVRIE_W::new(self)
    }
    #[doc = "Bit 3 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reocie(&mut self) -> REOCIE_W<3> {
        REOCIE_W::new(self)
    }
    #[doc = "Bit 4 - Regular data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rovrie(&mut self) -> ROVRIE_W<4> {
        ROVRIE_W::new(self)
    }
    #[doc = "Bits 8:9 - Reference voltage selection"]
    #[inline(always)]
    #[must_use]
    pub fn refv(&mut self) -> REFV_W<8> {
        REFV_W::new(self)
    }
    #[doc = "Bit 10 - Slow clock mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn slowck(&mut self) -> SLOWCK_W<10> {
        SLOWCK_W::new(self)
    }
    #[doc = "Bit 11 - Enter Standby mode when idle"]
    #[inline(always)]
    #[must_use]
    pub fn sbi(&mut self) -> SBI_W<11> {
        SBI_W::new(self)
    }
    #[doc = "Bit 12 - Enter power down mode when idle"]
    #[inline(always)]
    #[must_use]
    pub fn pdi(&mut self) -> PDI_W<12> {
        PDI_W::new(self)
    }
    #[doc = "Bit 14 - Launch a injected conversion synchronously with SDADC1"]
    #[inline(always)]
    #[must_use]
    pub fn jsync(&mut self) -> JSYNC_W<14> {
        JSYNC_W::new(self)
    }
    #[doc = "Bit 15 - Launch regular conversion synchronously with SDADC1"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<15> {
        RSYNC_W::new(self)
    }
    #[doc = "Bit 16 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    #[must_use]
    pub fn jdmaen(&mut self) -> JDMAEN_W<16> {
        JDMAEN_W::new(self)
    }
    #[doc = "Bit 17 - DMA channel enabled to read data for the regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn rdmaen(&mut self) -> RDMAEN_W<17> {
        RDMAEN_W::new(self)
    }
    #[doc = "Bit 31 - Initialization mode request"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<31> {
        INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
