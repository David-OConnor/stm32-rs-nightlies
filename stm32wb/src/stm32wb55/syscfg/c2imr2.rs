#[doc = "Register `C2IMR2` reader"]
pub struct R(crate::R<C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR2` writer"]
pub struct W(crate::W<C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR2_SPEC>;
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
impl From<crate::W<C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1_CH1_IM` reader - Peripheral DMA1 CH1 interrupt mask to CPU2"]
pub type DMA1_CH1_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH1_IM` writer - Peripheral DMA1 CH1 interrupt mask to CPU2"]
pub type DMA1_CH1_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA1_CH2_IM` reader - Peripheral DMA1 CH2 interrupt mask to CPU2"]
pub type DMA1_CH2_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH2_IM` writer - Peripheral DMA1 CH2 interrupt mask to CPU2"]
pub type DMA1_CH2_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA1_CH3_IM` reader - Peripheral DMA1 CH3 interrupt mask to CPU2"]
pub type DMA1_CH3_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH3_IM` writer - Peripheral DMA1 CH3 interrupt mask to CPU2"]
pub type DMA1_CH3_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA1_CH4_IM` reader - Peripheral DMA1 CH4 interrupt mask to CPU2"]
pub type DMA1_CH4_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH4_IM` writer - Peripheral DMA1 CH4 interrupt mask to CPU2"]
pub type DMA1_CH4_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA1_CH5_IM` reader - Peripheral DMA1 CH5 interrupt mask to CPU2"]
pub type DMA1_CH5_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH5_IM` writer - Peripheral DMA1 CH5 interrupt mask to CPU2"]
pub type DMA1_CH5_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA1_CH6_IM` reader - Peripheral DMA1 CH6 interrupt mask to CPU2"]
pub type DMA1_CH6_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH6_IM` writer - Peripheral DMA1 CH6 interrupt mask to CPU2"]
pub type DMA1_CH6_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA1_CH7_IM` reader - Peripheral DMA1 CH7 interrupt mask to CPU2"]
pub type DMA1_CH7_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA1_CH7_IM` writer - Peripheral DMA1 CH7 interrupt mask to CPU2"]
pub type DMA1_CH7_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH1_IM` reader - Peripheral DMA2 CH1 interrupt mask to CPU1"]
pub type DMA2_CH1_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH1_IM` writer - Peripheral DMA2 CH1 interrupt mask to CPU1"]
pub type DMA2_CH1_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH2_IM` reader - Peripheral DMA2 CH2 interrupt mask to CPU1"]
pub type DMA2_CH2_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH2_IM` writer - Peripheral DMA2 CH2 interrupt mask to CPU1"]
pub type DMA2_CH2_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH3_IM` reader - Peripheral DMA2 CH3 interrupt mask to CPU1"]
pub type DMA2_CH3_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH3_IM` writer - Peripheral DMA2 CH3 interrupt mask to CPU1"]
pub type DMA2_CH3_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH4_IM` reader - Peripheral DMA2 CH4 interrupt mask to CPU1"]
pub type DMA2_CH4_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH4_IM` writer - Peripheral DMA2 CH4 interrupt mask to CPU1"]
pub type DMA2_CH4_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH5_IM` reader - Peripheral DMA2 CH5 interrupt mask to CPU1"]
pub type DMA2_CH5_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH5_IM` writer - Peripheral DMA2 CH5 interrupt mask to CPU1"]
pub type DMA2_CH5_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH6_IM` reader - Peripheral DMA2 CH6 interrupt mask to CPU1"]
pub type DMA2_CH6_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH6_IM` writer - Peripheral DMA2 CH6 interrupt mask to CPU1"]
pub type DMA2_CH6_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMA2_CH7_IM` reader - Peripheral DMA2 CH7 interrupt mask to CPU1"]
pub type DMA2_CH7_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMA2_CH7_IM` writer - Peripheral DMA2 CH7 interrupt mask to CPU1"]
pub type DMA2_CH7_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `DMAM_UX1_IM` reader - Peripheral DMAM UX1 interrupt mask to CPU1"]
pub type DMAM_UX1_IM_R = crate::BitReader<bool>;
#[doc = "Field `DMAM_UX1_IM` writer - Peripheral DMAM UX1 interrupt mask to CPU1"]
pub type DMAM_UX1_IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `PVM1IM` reader - Peripheral PVM1IM interrupt mask to CPU1"]
pub type PVM1IM_R = crate::BitReader<bool>;
#[doc = "Field `PVM1IM` writer - Peripheral PVM1IM interrupt mask to CPU1"]
pub type PVM1IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `PVM3IM` reader - Peripheral PVM3IM interrupt mask to CPU1"]
pub type PVM3IM_R = crate::BitReader<bool>;
#[doc = "Field `PVM3IM` writer - Peripheral PVM3IM interrupt mask to CPU1"]
pub type PVM3IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `PVDIM` reader - Peripheral PVDIM interrupt mask to CPU1"]
pub type PVDIM_R = crate::BitReader<bool>;
#[doc = "Field `PVDIM` writer - Peripheral PVDIM interrupt mask to CPU1"]
pub type PVDIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `TSCIM` reader - Peripheral TSCIM interrupt mask to CPU1"]
pub type TSCIM_R = crate::BitReader<bool>;
#[doc = "Field `TSCIM` writer - Peripheral TSCIM interrupt mask to CPU1"]
pub type TSCIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
#[doc = "Field `LCDIM` reader - Peripheral LCDIM interrupt mask to CPU1"]
pub type LCDIM_R = crate::BitReader<bool>;
#[doc = "Field `LCDIM` writer - Peripheral LCDIM interrupt mask to CPU1"]
pub type LCDIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch1_im(&self) -> DMA1_CH1_IM_R {
        DMA1_CH1_IM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch2_im(&self) -> DMA1_CH2_IM_R {
        DMA1_CH2_IM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch3_im(&self) -> DMA1_CH3_IM_R {
        DMA1_CH3_IM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch4_im(&self) -> DMA1_CH4_IM_R {
        DMA1_CH4_IM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch5_im(&self) -> DMA1_CH5_IM_R {
        DMA1_CH5_IM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch6_im(&self) -> DMA1_CH6_IM_R {
        DMA1_CH6_IM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch7_im(&self) -> DMA1_CH7_IM_R {
        DMA1_CH7_IM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch1_im(&self) -> DMA2_CH1_IM_R {
        DMA2_CH1_IM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch2_im(&self) -> DMA2_CH2_IM_R {
        DMA2_CH2_IM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch3_im(&self) -> DMA2_CH3_IM_R {
        DMA2_CH3_IM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch4_im(&self) -> DMA2_CH4_IM_R {
        DMA2_CH4_IM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch5_im(&self) -> DMA2_CH5_IM_R {
        DMA2_CH5_IM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch6_im(&self) -> DMA2_CH6_IM_R {
        DMA2_CH6_IM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch7_im(&self) -> DMA2_CH7_IM_R {
        DMA2_CH7_IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dmam_ux1_im(&self) -> DMAM_UX1_IM_R {
        DMAM_UX1_IM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tscim(&self) -> TSCIM_R {
        TSCIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn lcdim(&self) -> LCDIM_R {
        LCDIM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch1_im(&mut self) -> DMA1_CH1_IM_W<0> {
        DMA1_CH1_IM_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch2_im(&mut self) -> DMA1_CH2_IM_W<1> {
        DMA1_CH2_IM_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch3_im(&mut self) -> DMA1_CH3_IM_W<2> {
        DMA1_CH3_IM_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch4_im(&mut self) -> DMA1_CH4_IM_W<3> {
        DMA1_CH4_IM_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch5_im(&mut self) -> DMA1_CH5_IM_W<4> {
        DMA1_CH5_IM_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch6_im(&mut self) -> DMA1_CH6_IM_W<5> {
        DMA1_CH6_IM_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn dma1_ch7_im(&mut self) -> DMA1_CH7_IM_W<6> {
        DMA1_CH7_IM_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch1_im(&mut self) -> DMA2_CH1_IM_W<8> {
        DMA2_CH1_IM_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch2_im(&mut self) -> DMA2_CH2_IM_W<9> {
        DMA2_CH2_IM_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch3_im(&mut self) -> DMA2_CH3_IM_W<10> {
        DMA2_CH3_IM_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch4_im(&mut self) -> DMA2_CH4_IM_W<11> {
        DMA2_CH4_IM_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch5_im(&mut self) -> DMA2_CH5_IM_W<12> {
        DMA2_CH5_IM_W::new(self)
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch6_im(&mut self) -> DMA2_CH6_IM_W<13> {
        DMA2_CH6_IM_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_ch7_im(&mut self) -> DMA2_CH7_IM_W<14> {
        DMA2_CH7_IM_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn dmam_ux1_im(&mut self) -> DMAM_UX1_IM_W<15> {
        DMAM_UX1_IM_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pvm1im(&mut self) -> PVM1IM_W<16> {
        PVM1IM_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pvm3im(&mut self) -> PVM3IM_W<18> {
        PVM3IM_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pvdim(&mut self) -> PVDIM_W<20> {
        PVDIM_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn tscim(&mut self) -> TSCIM_W<21> {
        TSCIM_W::new(self)
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn lcdim(&mut self) -> LCDIM_W<22> {
        LCDIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](index.html) module"]
pub struct C2IMR2_SPEC;
impl crate::RegisterSpec for C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr2::R](R) reader structure"]
impl crate::Readable for C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](W) writer structure"]
impl crate::Writable for C2IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2IMR2 to value 0"]
impl crate::Resettable for C2IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
