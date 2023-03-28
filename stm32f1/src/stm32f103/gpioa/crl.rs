#[doc = "Register `CRL` reader"]
pub struct R(crate::R<CRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRL` writer"]
pub struct W(crate::W<CRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRL_SPEC>;
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
impl From<crate::W<CRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE0` reader - Port n.0 mode bits"]
pub type MODE0_R = crate::FieldReader<u8, MODE0_A>;
#[doc = "Port n.0 mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: Output mode 10 MHz"]
    Output = 1,
    #[doc = "2: Output mode 2 MHz"]
    Output2 = 2,
    #[doc = "3: Output mode 50 MHz"]
    Output50 = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::Input,
            1 => MODE0_A::Output,
            2 => MODE0_A::Output2,
            3 => MODE0_A::Output50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Input`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0_A::Input
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE0_A::Output
    }
    #[doc = "Checks if the value of the field is `Output2`"]
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        *self == MODE0_A::Output2
    }
    #[doc = "Checks if the value of the field is `Output50`"]
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        *self == MODE0_A::Output50
    }
}
#[doc = "Field `MODE0` writer - Port n.0 mode bits"]
pub type MODE0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CRL_SPEC, u8, MODE0_A, 2, O>;
impl<'a, const O: u8> MODE0_W<'a, O> {
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0_A::Input)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0_A::Output)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0_A::Output2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0_A::Output50)
    }
}
#[doc = "Field `CNF0` reader - Port n.0 configuration bits"]
pub type CNF0_R = crate::FieldReader<u8, CNF0_A>;
#[doc = "Port n.0 configuration bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNF0_A {
    #[doc = "0: Analog mode / Push-Pull mode"]
    PushPull = 0,
    #[doc = "1: Floating input (reset state) / Open Drain-Mode"]
    OpenDrain = 1,
    #[doc = "2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    AltPushPull = 2,
    #[doc = "3: Alternate Function Open-Drain Mode"]
    AltOpenDrain = 3,
}
impl From<CNF0_A> for u8 {
    #[inline(always)]
    fn from(variant: CNF0_A) -> Self {
        variant as _
    }
}
impl CNF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNF0_A {
        match self.bits {
            0 => CNF0_A::PushPull,
            1 => CNF0_A::OpenDrain,
            2 => CNF0_A::AltPushPull,
            3 => CNF0_A::AltOpenDrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PushPull`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == CNF0_A::PushPull
    }
    #[doc = "Checks if the value of the field is `OpenDrain`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == CNF0_A::OpenDrain
    }
    #[doc = "Checks if the value of the field is `AltPushPull`"]
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        *self == CNF0_A::AltPushPull
    }
    #[doc = "Checks if the value of the field is `AltOpenDrain`"]
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        *self == CNF0_A::AltOpenDrain
    }
}
#[doc = "Field `CNF0` writer - Port n.0 configuration bits"]
pub type CNF0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CRL_SPEC, u8, CNF0_A, 2, O>;
impl<'a, const O: u8> CNF0_W<'a, O> {
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF0_A::PushPull)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF0_A::OpenDrain)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF0_A::AltPushPull)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF0_A::AltOpenDrain)
    }
}
#[doc = "Field `CNF1` reader - Port n.1 configuration bits"]
pub use CNF0_R as CNF1_R;
#[doc = "Field `CNF2` reader - Port n.2 configuration bits"]
pub use CNF0_R as CNF2_R;
#[doc = "Field `CNF3` reader - Port n.3 configuration bits"]
pub use CNF0_R as CNF3_R;
#[doc = "Field `CNF4` reader - Port n.4 configuration bits"]
pub use CNF0_R as CNF4_R;
#[doc = "Field `CNF5` reader - Port n.5 configuration bits"]
pub use CNF0_R as CNF5_R;
#[doc = "Field `CNF6` reader - Port n.6 configuration bits"]
pub use CNF0_R as CNF6_R;
#[doc = "Field `CNF7` reader - Port n.7 configuration bits"]
pub use CNF0_R as CNF7_R;
#[doc = "Field `CNF1` writer - Port n.1 configuration bits"]
pub use CNF0_W as CNF1_W;
#[doc = "Field `CNF2` writer - Port n.2 configuration bits"]
pub use CNF0_W as CNF2_W;
#[doc = "Field `CNF3` writer - Port n.3 configuration bits"]
pub use CNF0_W as CNF3_W;
#[doc = "Field `CNF4` writer - Port n.4 configuration bits"]
pub use CNF0_W as CNF4_W;
#[doc = "Field `CNF5` writer - Port n.5 configuration bits"]
pub use CNF0_W as CNF5_W;
#[doc = "Field `CNF6` writer - Port n.6 configuration bits"]
pub use CNF0_W as CNF6_W;
#[doc = "Field `CNF7` writer - Port n.7 configuration bits"]
pub use CNF0_W as CNF7_W;
#[doc = "Field `MODE1` reader - Port n.1 mode bits"]
pub use MODE0_R as MODE1_R;
#[doc = "Field `MODE2` reader - Port n.2 mode bits"]
pub use MODE0_R as MODE2_R;
#[doc = "Field `MODE3` reader - Port n.3 mode bits"]
pub use MODE0_R as MODE3_R;
#[doc = "Field `MODE4` reader - Port n.4 mode bits"]
pub use MODE0_R as MODE4_R;
#[doc = "Field `MODE5` reader - Port n.5 mode bits"]
pub use MODE0_R as MODE5_R;
#[doc = "Field `MODE6` reader - Port n.6 mode bits"]
pub use MODE0_R as MODE6_R;
#[doc = "Field `MODE7` reader - Port n.7 mode bits"]
pub use MODE0_R as MODE7_R;
#[doc = "Field `MODE1` writer - Port n.1 mode bits"]
pub use MODE0_W as MODE1_W;
#[doc = "Field `MODE2` writer - Port n.2 mode bits"]
pub use MODE0_W as MODE2_W;
#[doc = "Field `MODE3` writer - Port n.3 mode bits"]
pub use MODE0_W as MODE3_W;
#[doc = "Field `MODE4` writer - Port n.4 mode bits"]
pub use MODE0_W as MODE4_W;
#[doc = "Field `MODE5` writer - Port n.5 mode bits"]
pub use MODE0_W as MODE5_W;
#[doc = "Field `MODE6` writer - Port n.6 mode bits"]
pub use MODE0_W as MODE6_W;
#[doc = "Field `MODE7` writer - Port n.7 mode bits"]
pub use MODE0_W as MODE7_W;
impl R {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> CNF0_R {
        CNF0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> CNF1_R {
        CNF1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&self) -> CNF2_R {
        CNF2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&self) -> CNF3_R {
        CNF3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&self) -> CNF4_R {
        CNF4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&self) -> CNF5_R {
        CNF5_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&self) -> CNF6_R {
        CNF6_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&self) -> CNF7_R {
        CNF7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<0> {
        MODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf0(&mut self) -> CNF0_W<2> {
        CNF0_W::new(self)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<4> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf1(&mut self) -> CNF1_W<6> {
        CNF1_W::new(self)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<8> {
        MODE2_W::new(self)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf2(&mut self) -> CNF2_W<10> {
        CNF2_W::new(self)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<12> {
        MODE3_W::new(self)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf3(&mut self) -> CNF3_W<14> {
        CNF3_W::new(self)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<16> {
        MODE4_W::new(self)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf4(&mut self) -> CNF4_W<18> {
        CNF4_W::new(self)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<20> {
        MODE5_W::new(self)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf5(&mut self) -> CNF5_W<22> {
        CNF5_W::new(self)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<24> {
        MODE6_W::new(self)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf6(&mut self) -> CNF6_W<26> {
        CNF6_W::new(self)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<28> {
        MODE7_W::new(self)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf7(&mut self) -> CNF7_W<30> {
        CNF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register low (GPIOn_CRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crl](index.html) module"]
pub struct CRL_SPEC;
impl crate::RegisterSpec for CRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crl::R](R) reader structure"]
impl crate::Readable for CRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crl::W](W) writer structure"]
impl crate::Writable for CRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRL to value 0x4444_4444"]
impl crate::Resettable for CRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
