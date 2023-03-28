#[doc = "Register `RAMCFG_M2WPR1` reader"]
pub struct R(crate::R<RAMCFG_M2WPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_M2WPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_M2WPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_M2WPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMCFG_M2WPR1` writer"]
pub struct W(crate::W<RAMCFG_M2WPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCFG_M2WPR1_SPEC>;
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
impl From<crate::W<RAMCFG_M2WPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCFG_M2WPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P0WP_R = crate::BitReader<bool>;
#[doc = "Field `P0WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P0WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P1WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P1WP_R = crate::BitReader<bool>;
#[doc = "Field `P1WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P1WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P2WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P2WP_R = crate::BitReader<bool>;
#[doc = "Field `P2WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P2WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P3WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P3WP_R = crate::BitReader<bool>;
#[doc = "Field `P3WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P3WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P4WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P4WP_R = crate::BitReader<bool>;
#[doc = "Field `P4WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P4WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P5WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P5WP_R = crate::BitReader<bool>;
#[doc = "Field `P5WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P5WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P6WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P6WP_R = crate::BitReader<bool>;
#[doc = "Field `P6WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P6WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P7WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P7WP_R = crate::BitReader<bool>;
#[doc = "Field `P7WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P7WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P8WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P8WP_R = crate::BitReader<bool>;
#[doc = "Field `P8WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P8WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P9WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P9WP_R = crate::BitReader<bool>;
#[doc = "Field `P9WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P9WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P10WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P10WP_R = crate::BitReader<bool>;
#[doc = "Field `P10WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P10WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P11WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P11WP_R = crate::BitReader<bool>;
#[doc = "Field `P11WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P11WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P12WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P12WP_R = crate::BitReader<bool>;
#[doc = "Field `P12WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P12WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P13WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P13WP_R = crate::BitReader<bool>;
#[doc = "Field `P13WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P13WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P14WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P14WP_R = crate::BitReader<bool>;
#[doc = "Field `P14WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P14WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
#[doc = "Field `P15WP` reader - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P15WP_R = crate::BitReader<bool>;
#[doc = "Field `P15WP` writer - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
pub type P15WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M2WPR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p0wp(&self) -> P0WP_R {
        P0WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p1wp(&self) -> P1WP_R {
        P1WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p2wp(&self) -> P2WP_R {
        P2WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p3wp(&self) -> P3WP_R {
        P3WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p4wp(&self) -> P4WP_R {
        P4WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p5wp(&self) -> P5WP_R {
        P5WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p6wp(&self) -> P6WP_R {
        P6WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p7wp(&self) -> P7WP_R {
        P7WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p8wp(&self) -> P8WP_R {
        P8WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p9wp(&self) -> P9WP_R {
        P9WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p10wp(&self) -> P10WP_R {
        P10WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p11wp(&self) -> P11WP_R {
        P11WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p12wp(&self) -> P12WP_R {
        P12WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p13wp(&self) -> P13WP_R {
        P13WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p14wp(&self) -> P14WP_R {
        P14WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    pub fn p15wp(&self) -> P15WP_R {
        P15WP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p0wp(&mut self) -> P0WP_W<0> {
        P0WP_W::new(self)
    }
    #[doc = "Bit 1 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p1wp(&mut self) -> P1WP_W<1> {
        P1WP_W::new(self)
    }
    #[doc = "Bit 2 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p2wp(&mut self) -> P2WP_W<2> {
        P2WP_W::new(self)
    }
    #[doc = "Bit 3 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p3wp(&mut self) -> P3WP_W<3> {
        P3WP_W::new(self)
    }
    #[doc = "Bit 4 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p4wp(&mut self) -> P4WP_W<4> {
        P4WP_W::new(self)
    }
    #[doc = "Bit 5 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p5wp(&mut self) -> P5WP_W<5> {
        P5WP_W::new(self)
    }
    #[doc = "Bit 6 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p6wp(&mut self) -> P6WP_W<6> {
        P6WP_W::new(self)
    }
    #[doc = "Bit 7 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p7wp(&mut self) -> P7WP_W<7> {
        P7WP_W::new(self)
    }
    #[doc = "Bit 8 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p8wp(&mut self) -> P8WP_W<8> {
        P8WP_W::new(self)
    }
    #[doc = "Bit 9 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p9wp(&mut self) -> P9WP_W<9> {
        P9WP_W::new(self)
    }
    #[doc = "Bit 10 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p10wp(&mut self) -> P10WP_W<10> {
        P10WP_W::new(self)
    }
    #[doc = "Bit 11 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p11wp(&mut self) -> P11WP_W<11> {
        P11WP_W::new(self)
    }
    #[doc = "Bit 12 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p12wp(&mut self) -> P12WP_W<12> {
        P12WP_W::new(self)
    }
    #[doc = "Bit 13 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p13wp(&mut self) -> P13WP_W<13> {
        P13WP_W::new(self)
    }
    #[doc = "Bit 14 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p14wp(&mut self) -> P14WP_W<14> {
        P14WP_W::new(self)
    }
    #[doc = "Bit 15 - SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
    #[inline(always)]
    #[must_use]
    pub fn p15wp(&mut self) -> P15WP_W<15> {
        P15WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMCFG memory 2 write protection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m2wpr1](index.html) module"]
pub struct RAMCFG_M2WPR1_SPEC;
impl crate::RegisterSpec for RAMCFG_M2WPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg_m2wpr1::R](R) reader structure"]
impl crate::Readable for RAMCFG_M2WPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramcfg_m2wpr1::W](W) writer structure"]
impl crate::Writable for RAMCFG_M2WPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMCFG_M2WPR1 to value 0"]
impl crate::Resettable for RAMCFG_M2WPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
