#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GIF1` writer - GIF1"]
pub type GIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF1` writer - TCIF1"]
pub type TCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF1` writer - HTIF1"]
pub type HTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF1` writer - TEIF1"]
pub type TEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF2` writer - GIF2"]
pub type GIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF2` writer - TCIF2"]
pub type TCIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF2` writer - HTIF2"]
pub type HTIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF2` writer - TEIF2"]
pub type TEIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF3` writer - GIF3"]
pub type GIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF3` writer - TCIF3"]
pub type TCIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF3` writer - HTIF3"]
pub type HTIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF3` writer - TEIF3"]
pub type TEIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF4` writer - GIF4"]
pub type GIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF4` writer - TCIF4"]
pub type TCIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF4` writer - HTIF4"]
pub type HTIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF4` writer - TEIF4"]
pub type TEIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF5` writer - GIF5"]
pub type GIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF5` writer - TCIF5"]
pub type TCIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF5` writer - HTIF5"]
pub type HTIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF5` writer - TEIF5"]
pub type TEIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF6` writer - GIF6"]
pub type GIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF6` writer - TCIF6"]
pub type TCIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF6` writer - HTIF6"]
pub type HTIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF6` writer - TEIF6"]
pub type TEIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF7` writer - GIF7"]
pub type GIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF7` writer - TCIF7"]
pub type TCIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF7` writer - HTIF7"]
pub type HTIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF7` writer - TEIF7"]
pub type TEIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `GIF8` writer - GIF8"]
pub type GIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TCIF8` writer - TCIF8"]
pub type TCIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `HTIF8` writer - HTIF8"]
pub type HTIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `TEIF8` writer - TEIF8"]
pub type TEIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - GIF1"]
    #[inline(always)]
    #[must_use]
    pub fn gif1(&mut self) -> GIF1_W<0> {
        GIF1_W::new(self)
    }
    #[doc = "Bit 1 - TCIF1"]
    #[inline(always)]
    #[must_use]
    pub fn tcif1(&mut self) -> TCIF1_W<1> {
        TCIF1_W::new(self)
    }
    #[doc = "Bit 2 - HTIF1"]
    #[inline(always)]
    #[must_use]
    pub fn htif1(&mut self) -> HTIF1_W<2> {
        HTIF1_W::new(self)
    }
    #[doc = "Bit 3 - TEIF1"]
    #[inline(always)]
    #[must_use]
    pub fn teif1(&mut self) -> TEIF1_W<3> {
        TEIF1_W::new(self)
    }
    #[doc = "Bit 4 - GIF2"]
    #[inline(always)]
    #[must_use]
    pub fn gif2(&mut self) -> GIF2_W<4> {
        GIF2_W::new(self)
    }
    #[doc = "Bit 5 - TCIF2"]
    #[inline(always)]
    #[must_use]
    pub fn tcif2(&mut self) -> TCIF2_W<5> {
        TCIF2_W::new(self)
    }
    #[doc = "Bit 6 - HTIF2"]
    #[inline(always)]
    #[must_use]
    pub fn htif2(&mut self) -> HTIF2_W<6> {
        HTIF2_W::new(self)
    }
    #[doc = "Bit 7 - TEIF2"]
    #[inline(always)]
    #[must_use]
    pub fn teif2(&mut self) -> TEIF2_W<7> {
        TEIF2_W::new(self)
    }
    #[doc = "Bit 8 - GIF3"]
    #[inline(always)]
    #[must_use]
    pub fn gif3(&mut self) -> GIF3_W<8> {
        GIF3_W::new(self)
    }
    #[doc = "Bit 9 - TCIF3"]
    #[inline(always)]
    #[must_use]
    pub fn tcif3(&mut self) -> TCIF3_W<9> {
        TCIF3_W::new(self)
    }
    #[doc = "Bit 10 - HTIF3"]
    #[inline(always)]
    #[must_use]
    pub fn htif3(&mut self) -> HTIF3_W<10> {
        HTIF3_W::new(self)
    }
    #[doc = "Bit 11 - TEIF3"]
    #[inline(always)]
    #[must_use]
    pub fn teif3(&mut self) -> TEIF3_W<11> {
        TEIF3_W::new(self)
    }
    #[doc = "Bit 12 - GIF4"]
    #[inline(always)]
    #[must_use]
    pub fn gif4(&mut self) -> GIF4_W<12> {
        GIF4_W::new(self)
    }
    #[doc = "Bit 13 - TCIF4"]
    #[inline(always)]
    #[must_use]
    pub fn tcif4(&mut self) -> TCIF4_W<13> {
        TCIF4_W::new(self)
    }
    #[doc = "Bit 14 - HTIF4"]
    #[inline(always)]
    #[must_use]
    pub fn htif4(&mut self) -> HTIF4_W<14> {
        HTIF4_W::new(self)
    }
    #[doc = "Bit 15 - TEIF4"]
    #[inline(always)]
    #[must_use]
    pub fn teif4(&mut self) -> TEIF4_W<15> {
        TEIF4_W::new(self)
    }
    #[doc = "Bit 16 - GIF5"]
    #[inline(always)]
    #[must_use]
    pub fn gif5(&mut self) -> GIF5_W<16> {
        GIF5_W::new(self)
    }
    #[doc = "Bit 17 - TCIF5"]
    #[inline(always)]
    #[must_use]
    pub fn tcif5(&mut self) -> TCIF5_W<17> {
        TCIF5_W::new(self)
    }
    #[doc = "Bit 18 - HTIF5"]
    #[inline(always)]
    #[must_use]
    pub fn htif5(&mut self) -> HTIF5_W<18> {
        HTIF5_W::new(self)
    }
    #[doc = "Bit 19 - TEIF5"]
    #[inline(always)]
    #[must_use]
    pub fn teif5(&mut self) -> TEIF5_W<19> {
        TEIF5_W::new(self)
    }
    #[doc = "Bit 20 - GIF6"]
    #[inline(always)]
    #[must_use]
    pub fn gif6(&mut self) -> GIF6_W<20> {
        GIF6_W::new(self)
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    #[must_use]
    pub fn tcif6(&mut self) -> TCIF6_W<21> {
        TCIF6_W::new(self)
    }
    #[doc = "Bit 22 - HTIF6"]
    #[inline(always)]
    #[must_use]
    pub fn htif6(&mut self) -> HTIF6_W<22> {
        HTIF6_W::new(self)
    }
    #[doc = "Bit 23 - TEIF6"]
    #[inline(always)]
    #[must_use]
    pub fn teif6(&mut self) -> TEIF6_W<23> {
        TEIF6_W::new(self)
    }
    #[doc = "Bit 24 - GIF7"]
    #[inline(always)]
    #[must_use]
    pub fn gif7(&mut self) -> GIF7_W<24> {
        GIF7_W::new(self)
    }
    #[doc = "Bit 25 - TCIF7"]
    #[inline(always)]
    #[must_use]
    pub fn tcif7(&mut self) -> TCIF7_W<25> {
        TCIF7_W::new(self)
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    #[must_use]
    pub fn htif7(&mut self) -> HTIF7_W<26> {
        HTIF7_W::new(self)
    }
    #[doc = "Bit 27 - TEIF7"]
    #[inline(always)]
    #[must_use]
    pub fn teif7(&mut self) -> TEIF7_W<27> {
        TEIF7_W::new(self)
    }
    #[doc = "Bit 28 - GIF8"]
    #[inline(always)]
    #[must_use]
    pub fn gif8(&mut self) -> GIF8_W<28> {
        GIF8_W::new(self)
    }
    #[doc = "Bit 29 - TCIF8"]
    #[inline(always)]
    #[must_use]
    pub fn tcif8(&mut self) -> TCIF8_W<29> {
        TCIF8_W::new(self)
    }
    #[doc = "Bit 30 - HTIF8"]
    #[inline(always)]
    #[must_use]
    pub fn htif8(&mut self) -> HTIF8_W<30> {
        HTIF8_W::new(self)
    }
    #[doc = "Bit 31 - TEIF8"]
    #[inline(always)]
    #[must_use]
    pub fn teif8(&mut self) -> TEIF8_W<31> {
        TEIF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
