#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSWU` reader - Master Timer Software update"]
pub type MSWU_R = crate::BitReader<MSWU_A>;
#[doc = "Master Timer Software update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSWU_A {
    #[doc = "1: Force immediate update"]
    Update = 1,
}
impl From<MSWU_A> for bool {
    #[inline(always)]
    fn from(variant: MSWU_A) -> Self {
        variant as u8 != 0
    }
}
impl MSWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSWU_A> {
        match self.bits {
            true => Some(MSWU_A::Update),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Update`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MSWU_A::Update
    }
}
#[doc = "Field `MSWU` writer - Master Timer Software update"]
pub type MSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, MSWU_A, O>;
impl<'a, const O: u8> MSWU_W<'a, O> {
    #[doc = "Force immediate update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MSWU_A::Update)
    }
}
#[doc = "Field `TASWU` reader - Timer A Software update"]
pub use MSWU_R as TASWU_R;
#[doc = "Field `TBSWU` reader - Timer B Software Update"]
pub use MSWU_R as TBSWU_R;
#[doc = "Field `TCSWU` reader - Timer C Software Update"]
pub use MSWU_R as TCSWU_R;
#[doc = "Field `TDSWU` reader - Timer D Software Update"]
pub use MSWU_R as TDSWU_R;
#[doc = "Field `TESWU` reader - Timer E Software Update"]
pub use MSWU_R as TESWU_R;
#[doc = "Field `TASWU` writer - Timer A Software update"]
pub use MSWU_W as TASWU_W;
#[doc = "Field `TBSWU` writer - Timer B Software Update"]
pub use MSWU_W as TBSWU_W;
#[doc = "Field `TCSWU` writer - Timer C Software Update"]
pub use MSWU_W as TCSWU_W;
#[doc = "Field `TDSWU` writer - Timer D Software Update"]
pub use MSWU_W as TDSWU_W;
#[doc = "Field `TESWU` writer - Timer E Software Update"]
pub use MSWU_W as TESWU_W;
#[doc = "Field `MRST` reader - Master Counter software reset"]
pub type MRST_R = crate::BitReader<MRST_A>;
#[doc = "Master Counter software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRST_A {
    #[doc = "1: Reset timer"]
    Reset = 1,
}
impl From<MRST_A> for bool {
    #[inline(always)]
    fn from(variant: MRST_A) -> Self {
        variant as u8 != 0
    }
}
impl MRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MRST_A> {
        match self.bits {
            true => Some(MRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MRST_A::Reset
    }
}
#[doc = "Field `MRST` writer - Master Counter software reset"]
pub type MRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, MRST_A, O>;
impl<'a, const O: u8> MRST_W<'a, O> {
    #[doc = "Reset timer"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MRST_A::Reset)
    }
}
#[doc = "Field `TARST` reader - Timer A counter software reset"]
pub use MRST_R as TARST_R;
#[doc = "Field `TBRST` reader - Timer B counter software reset"]
pub use MRST_R as TBRST_R;
#[doc = "Field `TCRST` reader - Timer C counter software reset"]
pub use MRST_R as TCRST_R;
#[doc = "Field `TDRST` reader - Timer D counter software reset"]
pub use MRST_R as TDRST_R;
#[doc = "Field `TERST` reader - Timer E counter software reset"]
pub use MRST_R as TERST_R;
#[doc = "Field `TARST` writer - Timer A counter software reset"]
pub use MRST_W as TARST_W;
#[doc = "Field `TBRST` writer - Timer B counter software reset"]
pub use MRST_W as TBRST_W;
#[doc = "Field `TCRST` writer - Timer C counter software reset"]
pub use MRST_W as TCRST_W;
#[doc = "Field `TDRST` writer - Timer D counter software reset"]
pub use MRST_W as TDRST_W;
#[doc = "Field `TERST` writer - Timer E counter software reset"]
pub use MRST_W as TERST_W;
impl R {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&self) -> TASWU_R {
        TASWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&self) -> TBSWU_R {
        TBSWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&self) -> TCSWU_R {
        TCSWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&self) -> TDSWU_R {
        TDSWU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&self) -> TESWU_R {
        TESWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    #[must_use]
    pub fn mswu(&mut self) -> MSWU_W<0> {
        MSWU_W::new(self)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    #[must_use]
    pub fn taswu(&mut self) -> TASWU_W<1> {
        TASWU_W::new(self)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tbswu(&mut self) -> TBSWU_W<2> {
        TBSWU_W::new(self)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tcswu(&mut self) -> TCSWU_W<3> {
        TCSWU_W::new(self)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tdswu(&mut self) -> TDSWU_W<4> {
        TDSWU_W::new(self)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn teswu(&mut self) -> TESWU_W<5> {
        TESWU_W::new(self)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn mrst(&mut self) -> MRST_W<8> {
        MRST_W::new(self)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<9> {
        TARST_W::new(self)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<10> {
        TBRST_W::new(self)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<11> {
        TCRST_W::new(self)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<12> {
        TDRST_W::new(self)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<13> {
        TERST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
