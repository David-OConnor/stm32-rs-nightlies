#[doc = "Register `SWPR` writer"]
pub struct W(crate::W<SWPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPR_SPEC>;
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
impl From<crate::W<SWPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "P0WP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0WP_AW {
    #[doc = "0: Write protection of SRAM2 page x is disabled"]
    Disabled = 0,
    #[doc = "1: Write protection of SRAM2 page x is enabled"]
    Enabled = 1,
}
impl From<P0WP_AW> for bool {
    #[inline(always)]
    fn from(variant: P0WP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0WP` writer - P0WP"]
pub type P0WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR_SPEC, P0WP_AW, O>;
impl<'a, const O: u8> P0WP_W<'a, O> {
    #[doc = "Write protection of SRAM2 page x is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0WP_AW::Disabled)
    }
    #[doc = "Write protection of SRAM2 page x is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(P0WP_AW::Enabled)
    }
}
#[doc = "Field `P1WP` writer - P1WP"]
pub use P0WP_W as P1WP_W;
#[doc = "Field `P2WP` writer - P2WP"]
pub use P0WP_W as P2WP_W;
#[doc = "Field `P3WP` writer - P3WP"]
pub use P0WP_W as P3WP_W;
#[doc = "Field `P4WP` writer - P4WP"]
pub use P0WP_W as P4WP_W;
#[doc = "Field `P5WP` writer - P5WP"]
pub use P0WP_W as P5WP_W;
#[doc = "Field `P6WP` writer - P6WP"]
pub use P0WP_W as P6WP_W;
#[doc = "Field `P7WP` writer - P7WP"]
pub use P0WP_W as P7WP_W;
#[doc = "Field `P8WP` writer - P8WP"]
pub use P0WP_W as P8WP_W;
#[doc = "Field `P9WP` writer - P9WP"]
pub use P0WP_W as P9WP_W;
#[doc = "Field `P10WP` writer - P10WP"]
pub use P0WP_W as P10WP_W;
#[doc = "Field `P11WP` writer - P11WP"]
pub use P0WP_W as P11WP_W;
#[doc = "Field `P12WP` writer - P12WP"]
pub use P0WP_W as P12WP_W;
#[doc = "Field `P13WP` writer - P13WP"]
pub use P0WP_W as P13WP_W;
#[doc = "Field `P14WP` writer - P14WP"]
pub use P0WP_W as P14WP_W;
#[doc = "Field `P15WP` writer - P15WP"]
pub use P0WP_W as P15WP_W;
#[doc = "Field `P16WP` writer - P16WP"]
pub use P0WP_W as P16WP_W;
#[doc = "Field `P17WP` writer - P17WP"]
pub use P0WP_W as P17WP_W;
#[doc = "Field `P18WP` writer - P18WP"]
pub use P0WP_W as P18WP_W;
#[doc = "Field `P19WP` writer - P19WP"]
pub use P0WP_W as P19WP_W;
#[doc = "Field `P20WP` writer - P20WP"]
pub use P0WP_W as P20WP_W;
#[doc = "Field `P21WP` writer - P21WP"]
pub use P0WP_W as P21WP_W;
#[doc = "Field `P22WP` writer - P22WP"]
pub use P0WP_W as P22WP_W;
#[doc = "Field `P23WP` writer - P23WP"]
pub use P0WP_W as P23WP_W;
#[doc = "Field `P24WP` writer - P24WP"]
pub use P0WP_W as P24WP_W;
#[doc = "Field `P25WP` writer - P25WP"]
pub use P0WP_W as P25WP_W;
#[doc = "Field `P26WP` writer - P26WP"]
pub use P0WP_W as P26WP_W;
#[doc = "Field `P27WP` writer - P27WP"]
pub use P0WP_W as P27WP_W;
#[doc = "Field `P28WP` writer - P28WP"]
pub use P0WP_W as P28WP_W;
#[doc = "Field `P29WP` writer - P29WP"]
pub use P0WP_W as P29WP_W;
#[doc = "Field `P30WP` writer - P30WP"]
pub use P0WP_W as P30WP_W;
#[doc = "Field `P31WP` writer - SRAM2 page 31 write protection"]
pub use P0WP_W as P31WP_W;
impl W {
    #[doc = "Bit 0 - P0WP"]
    #[inline(always)]
    #[must_use]
    pub fn p0wp(&mut self) -> P0WP_W<0> {
        P0WP_W::new(self)
    }
    #[doc = "Bit 1 - P1WP"]
    #[inline(always)]
    #[must_use]
    pub fn p1wp(&mut self) -> P1WP_W<1> {
        P1WP_W::new(self)
    }
    #[doc = "Bit 2 - P2WP"]
    #[inline(always)]
    #[must_use]
    pub fn p2wp(&mut self) -> P2WP_W<2> {
        P2WP_W::new(self)
    }
    #[doc = "Bit 3 - P3WP"]
    #[inline(always)]
    #[must_use]
    pub fn p3wp(&mut self) -> P3WP_W<3> {
        P3WP_W::new(self)
    }
    #[doc = "Bit 4 - P4WP"]
    #[inline(always)]
    #[must_use]
    pub fn p4wp(&mut self) -> P4WP_W<4> {
        P4WP_W::new(self)
    }
    #[doc = "Bit 5 - P5WP"]
    #[inline(always)]
    #[must_use]
    pub fn p5wp(&mut self) -> P5WP_W<5> {
        P5WP_W::new(self)
    }
    #[doc = "Bit 6 - P6WP"]
    #[inline(always)]
    #[must_use]
    pub fn p6wp(&mut self) -> P6WP_W<6> {
        P6WP_W::new(self)
    }
    #[doc = "Bit 7 - P7WP"]
    #[inline(always)]
    #[must_use]
    pub fn p7wp(&mut self) -> P7WP_W<7> {
        P7WP_W::new(self)
    }
    #[doc = "Bit 8 - P8WP"]
    #[inline(always)]
    #[must_use]
    pub fn p8wp(&mut self) -> P8WP_W<8> {
        P8WP_W::new(self)
    }
    #[doc = "Bit 9 - P9WP"]
    #[inline(always)]
    #[must_use]
    pub fn p9wp(&mut self) -> P9WP_W<9> {
        P9WP_W::new(self)
    }
    #[doc = "Bit 10 - P10WP"]
    #[inline(always)]
    #[must_use]
    pub fn p10wp(&mut self) -> P10WP_W<10> {
        P10WP_W::new(self)
    }
    #[doc = "Bit 11 - P11WP"]
    #[inline(always)]
    #[must_use]
    pub fn p11wp(&mut self) -> P11WP_W<11> {
        P11WP_W::new(self)
    }
    #[doc = "Bit 12 - P12WP"]
    #[inline(always)]
    #[must_use]
    pub fn p12wp(&mut self) -> P12WP_W<12> {
        P12WP_W::new(self)
    }
    #[doc = "Bit 13 - P13WP"]
    #[inline(always)]
    #[must_use]
    pub fn p13wp(&mut self) -> P13WP_W<13> {
        P13WP_W::new(self)
    }
    #[doc = "Bit 14 - P14WP"]
    #[inline(always)]
    #[must_use]
    pub fn p14wp(&mut self) -> P14WP_W<14> {
        P14WP_W::new(self)
    }
    #[doc = "Bit 15 - P15WP"]
    #[inline(always)]
    #[must_use]
    pub fn p15wp(&mut self) -> P15WP_W<15> {
        P15WP_W::new(self)
    }
    #[doc = "Bit 16 - P16WP"]
    #[inline(always)]
    #[must_use]
    pub fn p16wp(&mut self) -> P16WP_W<16> {
        P16WP_W::new(self)
    }
    #[doc = "Bit 17 - P17WP"]
    #[inline(always)]
    #[must_use]
    pub fn p17wp(&mut self) -> P17WP_W<17> {
        P17WP_W::new(self)
    }
    #[doc = "Bit 18 - P18WP"]
    #[inline(always)]
    #[must_use]
    pub fn p18wp(&mut self) -> P18WP_W<18> {
        P18WP_W::new(self)
    }
    #[doc = "Bit 19 - P19WP"]
    #[inline(always)]
    #[must_use]
    pub fn p19wp(&mut self) -> P19WP_W<19> {
        P19WP_W::new(self)
    }
    #[doc = "Bit 20 - P20WP"]
    #[inline(always)]
    #[must_use]
    pub fn p20wp(&mut self) -> P20WP_W<20> {
        P20WP_W::new(self)
    }
    #[doc = "Bit 21 - P21WP"]
    #[inline(always)]
    #[must_use]
    pub fn p21wp(&mut self) -> P21WP_W<21> {
        P21WP_W::new(self)
    }
    #[doc = "Bit 22 - P22WP"]
    #[inline(always)]
    #[must_use]
    pub fn p22wp(&mut self) -> P22WP_W<22> {
        P22WP_W::new(self)
    }
    #[doc = "Bit 23 - P23WP"]
    #[inline(always)]
    #[must_use]
    pub fn p23wp(&mut self) -> P23WP_W<23> {
        P23WP_W::new(self)
    }
    #[doc = "Bit 24 - P24WP"]
    #[inline(always)]
    #[must_use]
    pub fn p24wp(&mut self) -> P24WP_W<24> {
        P24WP_W::new(self)
    }
    #[doc = "Bit 25 - P25WP"]
    #[inline(always)]
    #[must_use]
    pub fn p25wp(&mut self) -> P25WP_W<25> {
        P25WP_W::new(self)
    }
    #[doc = "Bit 26 - P26WP"]
    #[inline(always)]
    #[must_use]
    pub fn p26wp(&mut self) -> P26WP_W<26> {
        P26WP_W::new(self)
    }
    #[doc = "Bit 27 - P27WP"]
    #[inline(always)]
    #[must_use]
    pub fn p27wp(&mut self) -> P27WP_W<27> {
        P27WP_W::new(self)
    }
    #[doc = "Bit 28 - P28WP"]
    #[inline(always)]
    #[must_use]
    pub fn p28wp(&mut self) -> P28WP_W<28> {
        P28WP_W::new(self)
    }
    #[doc = "Bit 29 - P29WP"]
    #[inline(always)]
    #[must_use]
    pub fn p29wp(&mut self) -> P29WP_W<29> {
        P29WP_W::new(self)
    }
    #[doc = "Bit 30 - P30WP"]
    #[inline(always)]
    #[must_use]
    pub fn p30wp(&mut self) -> P30WP_W<30> {
        P30WP_W::new(self)
    }
    #[doc = "Bit 31 - SRAM2 page 31 write protection"]
    #[inline(always)]
    #[must_use]
    pub fn p31wp(&mut self) -> P31WP_W<31> {
        P31WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SWPR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr](index.html) module"]
pub struct SWPR_SPEC;
impl crate::RegisterSpec for SWPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swpr::W](W) writer structure"]
impl crate::Writable for SWPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWPR to value 0"]
impl crate::Resettable for SWPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
