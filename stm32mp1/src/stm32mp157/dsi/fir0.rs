#[doc = "Register `FIR0` writer"]
pub struct W(crate::W<FIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR0_SPEC>;
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
impl From<crate::W<FIR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAE0` writer - FAE0"]
pub type FAE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE1` writer - FAE1"]
pub type FAE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE2` writer - FAE2"]
pub type FAE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE3` writer - FAE3"]
pub type FAE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE4` writer - FAE4"]
pub type FAE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE5` writer - FAE5"]
pub type FAE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE6` writer - FAE6"]
pub type FAE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE7` writer - FAE7"]
pub type FAE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE8` writer - FAE8"]
pub type FAE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE9` writer - FAE9"]
pub type FAE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE10` writer - FAE10"]
pub type FAE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE11` writer - FAE11"]
pub type FAE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE12` writer - FAE12"]
pub type FAE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE13` writer - FAE13"]
pub type FAE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE14` writer - FAE14"]
pub type FAE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE15` writer - FAE15"]
pub type FAE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE0` writer - FPE0"]
pub type FPE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE1` writer - FPE1"]
pub type FPE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE2` writer - FPE2"]
pub type FPE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE3` writer - FPE3"]
pub type FPE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE4` writer - FPE4"]
pub type FPE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - FAE0"]
    #[inline(always)]
    #[must_use]
    pub fn fae0(&mut self) -> FAE0_W<0> {
        FAE0_W::new(self)
    }
    #[doc = "Bit 1 - FAE1"]
    #[inline(always)]
    #[must_use]
    pub fn fae1(&mut self) -> FAE1_W<1> {
        FAE1_W::new(self)
    }
    #[doc = "Bit 2 - FAE2"]
    #[inline(always)]
    #[must_use]
    pub fn fae2(&mut self) -> FAE2_W<2> {
        FAE2_W::new(self)
    }
    #[doc = "Bit 3 - FAE3"]
    #[inline(always)]
    #[must_use]
    pub fn fae3(&mut self) -> FAE3_W<3> {
        FAE3_W::new(self)
    }
    #[doc = "Bit 4 - FAE4"]
    #[inline(always)]
    #[must_use]
    pub fn fae4(&mut self) -> FAE4_W<4> {
        FAE4_W::new(self)
    }
    #[doc = "Bit 5 - FAE5"]
    #[inline(always)]
    #[must_use]
    pub fn fae5(&mut self) -> FAE5_W<5> {
        FAE5_W::new(self)
    }
    #[doc = "Bit 6 - FAE6"]
    #[inline(always)]
    #[must_use]
    pub fn fae6(&mut self) -> FAE6_W<6> {
        FAE6_W::new(self)
    }
    #[doc = "Bit 7 - FAE7"]
    #[inline(always)]
    #[must_use]
    pub fn fae7(&mut self) -> FAE7_W<7> {
        FAE7_W::new(self)
    }
    #[doc = "Bit 8 - FAE8"]
    #[inline(always)]
    #[must_use]
    pub fn fae8(&mut self) -> FAE8_W<8> {
        FAE8_W::new(self)
    }
    #[doc = "Bit 9 - FAE9"]
    #[inline(always)]
    #[must_use]
    pub fn fae9(&mut self) -> FAE9_W<9> {
        FAE9_W::new(self)
    }
    #[doc = "Bit 10 - FAE10"]
    #[inline(always)]
    #[must_use]
    pub fn fae10(&mut self) -> FAE10_W<10> {
        FAE10_W::new(self)
    }
    #[doc = "Bit 11 - FAE11"]
    #[inline(always)]
    #[must_use]
    pub fn fae11(&mut self) -> FAE11_W<11> {
        FAE11_W::new(self)
    }
    #[doc = "Bit 12 - FAE12"]
    #[inline(always)]
    #[must_use]
    pub fn fae12(&mut self) -> FAE12_W<12> {
        FAE12_W::new(self)
    }
    #[doc = "Bit 13 - FAE13"]
    #[inline(always)]
    #[must_use]
    pub fn fae13(&mut self) -> FAE13_W<13> {
        FAE13_W::new(self)
    }
    #[doc = "Bit 14 - FAE14"]
    #[inline(always)]
    #[must_use]
    pub fn fae14(&mut self) -> FAE14_W<14> {
        FAE14_W::new(self)
    }
    #[doc = "Bit 15 - FAE15"]
    #[inline(always)]
    #[must_use]
    pub fn fae15(&mut self) -> FAE15_W<15> {
        FAE15_W::new(self)
    }
    #[doc = "Bit 16 - FPE0"]
    #[inline(always)]
    #[must_use]
    pub fn fpe0(&mut self) -> FPE0_W<16> {
        FPE0_W::new(self)
    }
    #[doc = "Bit 17 - FPE1"]
    #[inline(always)]
    #[must_use]
    pub fn fpe1(&mut self) -> FPE1_W<17> {
        FPE1_W::new(self)
    }
    #[doc = "Bit 18 - FPE2"]
    #[inline(always)]
    #[must_use]
    pub fn fpe2(&mut self) -> FPE2_W<18> {
        FPE2_W::new(self)
    }
    #[doc = "Bit 19 - FPE3"]
    #[inline(always)]
    #[must_use]
    pub fn fpe3(&mut self) -> FPE3_W<19> {
        FPE3_W::new(self)
    }
    #[doc = "Bit 20 - FPE4"]
    #[inline(always)]
    #[must_use]
    pub fn fpe4(&mut self) -> FPE4_W<20> {
        FPE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host force interrupt register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir0](index.html) module"]
pub struct FIR0_SPEC;
impl crate::RegisterSpec for FIR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fir0::W](W) writer structure"]
impl crate::Writable for FIR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIR0 to value 0"]
impl crate::Resettable for FIR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
